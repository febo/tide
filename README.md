<h1 align="center">
  <code>tide</code>
</h1>
<p align="center">
  <img width="400" alt="playground" src="https://github.com/user-attachments/assets/2b80e13a-9d9d-4689-8a67-631fcb86cf8d" />
</p>

<p align="center">
  Benchmarking serialization crates for Solana programs.
</p>

## Overview

Manipulating account data is one of the main operations of a Solana program. At the same time,
it can be one of the most expensive.

`tide` is a benchmark suite comparing popular serialization crates, from zero-copy approaches to borsh (used by Anchor). The benchmark demonstrates **full deserialization** (baseline) as well as **partial operations** (reading/updating individual fields).

### What Makes This Benchmark Unique

Most benchmarks only test full serialization/deserialization. In reality, Solana programs often only need to:
- **Read a single field** (e.g., check the owner)
- **Update a single field** (e.g., increment a balance)

This benchmark reveals the **true cost difference** between:
- **Zero-copy libraries** (transmute, bytemuck, zerocopy) - can access individual fields directly
- **Serialization libraries** (borsh, bincode, wincode) - must deserialize the entire struct

## Key Findings

The results show dramatic performance differences for partial operations:

| Operation | Transmute | Bytemuck | Zerocopy | Bincode | Borsh |
|-----------|-----------|----------|----------|---------|-------|
| **Full** | 51 CUs | 55 CUs | 55 CUs | 320 CUs | 813 CUs |
| **Read Owner** | 43 CUs | 46 CUs | 46 CUs | 244 CUs | 468 CUs |
| **Update Amount** | 29 CUs | 36 CUs | 36 CUs | 312 CUs | 794 CUs |

**Insight:** For partial updates, zero-copy libraries are **10-25x more efficient** than serialization libraries.

The repository uses a base `Account` struct that resembles a token account (`160` bytes):
```rust
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,

    /// The owner of this account.
    pub owner: Pubkey,

    /// The amount of tokens this account holds.
    pub amount: u64,

    /// The delegate for this account.
    pub delegate: Pubkey,

    /// The account's state.
    pub state: u8,

    /// Padding to align fields.
    _padding: [u8; 7],

    /// Native token amount.
    pub native_amount: u64,

    /// The amount delegated.
    pub delegated_amount: u64,

    /// The close authority.
    pub close_authority: Pubkey,
}
```

Programs read and update data on the account. It reads a `Pubkey`, owner of the account; the update consists of changing an `u8` and `u64`, state and amount fields.

## Building and Running

A [`Makefile`](./Makefile) is provided with basic commands to:
* `all`: build all programs &mdash; his is required before running the benchmark.
* `bench-tests`: run the benchmark.
* `build-sbf-%`: build an individual program &mdash; `%` is the serialization name.
* `test-benchmark`: run the tests.

To execute the benchmark, it is first necessary to build all programs:
```bash
make all
```


Followed by:
```bash
make bench
```

After the execution, mollusk will report the compute units in a `compute_units.md`
located at `./target/benches`.

### Sample Results

The benchmark tests three scenarios for each library:

```
| Name                      | CUs |
|---------------------------|-----|
| bincode::full             | 320 |
| bincode::read_owner       | 244 |
| bincode::update_amount    | 312 |
| borsh::full               | 813 |
| borsh::read_owner         | 468 |
| borsh::update_amount      | 794 |
| bytemuck::full            | 55  |
| bytemuck::read_owner      | 46  |
| bytemuck::update_amount   | 36  |
| transmute::full           | 51  |
| transmute::read_owner     | 43  |
| transmute::update_amount  | 29  |
| wincode::full             | 77  |
| wincode::read_owner       | 55  |
| wincode::update_amount    | 32  |
| zerocopy::full            | 55  |
| zerocopy::read_owner      | 46  |
| zerocopy::update_amount   | 36  |
```

**Notable:** Zero-copy libraries (transmute, bytemuck, zerocopy) excel at partial operations, while serialization libraries (bincode, borsh) must process the entire struct even for single-field updates.

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)

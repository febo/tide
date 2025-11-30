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

`tide` is a small benchmark of popular serialization crates, from zero-copy ones to borsh, which is used by Anchor.
It is not indended to be exhaustive, but to give an idea how they are used and their performace in
term of compute units consumed.

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

    /// Padding bytes.
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
* `bench`: run the benchmark.
* `build-sbf-%`: build an individual program &mdash; `%` is the serialization program name (e.g., `programs-transmute`).
* `tests`: run the tests.

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
```
| Name               | CUs | Delta |
|--------------------|-----|-------|
| bincode::account   | 214 |  --   |
| borsh::account     | 617 |  --   |
| bytemuck::account  | 40  |  --   |
| transmute::account | 36  |  --   |
| wincode::account   | 62  |  --   |
| zerocopy::account  | 40  |  --   |
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)

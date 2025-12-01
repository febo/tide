#![feature(test)]

extern crate test;

use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_address::Address;
use test::Bencher;
use tide::{instruction, setup};

const DEFAULT_LOADER_KEY: Address = solana_sdk_ids::bpf_loader_upgradeable::ID;

#[cfg(test)]
#[bench]
fn run(_bencher: &mut Bencher) {
    // bincode V1
    let bincode_v1_id = Address::from_str_const("BincodeV11111111111111111111111111111111111");
    let mut mollusk = setup(&bincode_v1_id, "bincode_v1_program");

    // bincode V2
    let bincode_v2_id = Address::from_str_const("BincodeV21111111111111111111111111111111111");
    mollusk.add_program(&bincode_v2_id, "bincode_v2_program", &DEFAULT_LOADER_KEY);

    // borsh
    let borsh_id = Address::from_str_const("Borsh111111111111111111111111111111111111111");
    mollusk.add_program(&borsh_id, "borsh_program", &DEFAULT_LOADER_KEY);

    // bytemuck
    let bytemuck_id = Address::from_str_const("Bytemuck111111111111111111111111111111111111");
    mollusk.add_program(&bytemuck_id, "bytemuck_program", &DEFAULT_LOADER_KEY);

    // transmute
    let transmute_id = Address::from_str_const("Transmute1111111111111111111111111111111111");
    mollusk.add_program(&transmute_id, "transmute_program", &DEFAULT_LOADER_KEY);

    // wincode
    let wincode_id = Address::from_str_const("Wincode111111111111111111111111111111111111");
    mollusk.add_program(&wincode_id, "wincode_program", &DEFAULT_LOADER_KEY);

    // zerocopy
    let zerocopy_id = Address::from_str_const("Zerocopy11111111111111111111111111111111111");
    mollusk.add_program(&zerocopy_id, "zerocopy_program", &DEFAULT_LOADER_KEY);

    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    let (ix, accounts) = instruction(&bincode_v1_id);
    bencher = bencher.bench(("bincode_v1::account", &ix, &accounts));

    let (ix, accounts) = instruction(&bincode_v2_id);
    bencher = bencher.bench(("bincode_v2::account", &ix, &accounts));

    let (ix, accounts) = instruction(&borsh_id);
    bencher = bencher.bench(("borsh::account", &ix, &accounts));

    let (ix, accounts) = instruction(&bytemuck_id);
    bencher = bencher.bench(("bytemuck::account", &ix, &accounts));

    let (ix, accounts) = instruction(&transmute_id);
    bencher = bencher.bench(("transmute::account", &ix, &accounts));

    let (ix, accounts) = instruction(&wincode_id);
    bencher = bencher.bench(("wincode::account", &ix, &accounts));

    let (ix, accounts) = instruction(&zerocopy_id);
    bencher = bencher.bench(("zerocopy::account", &ix, &accounts));

    bencher.execute();
}

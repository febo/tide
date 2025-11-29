#![feature(test)]

extern crate test;

use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_address::Address;
use test::Bencher;
use tide::{instruction_full, instruction_read_owner, instruction_update_amount, setup};

const DEFAULT_LOADER_KEY: Address = solana_sdk_ids::bpf_loader_upgradeable::ID;

#[cfg(test)]
#[bench]
fn run(_bencher: &mut Bencher) {
    // bincode
    let bincode_id = Address::from_str_const("Bincode111111111111111111111111111111111111");
    let mut mollusk = setup(&bincode_id, "bincode_program");

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

    let (ix, accounts) = instruction_full(&bincode_id);
    bencher = bencher.bench(("bincode::full", &ix, &accounts));

    let (ix, accounts) = instruction_full(&borsh_id);
    bencher = bencher.bench(("borsh::full", &ix, &accounts));

    let (ix, accounts) = instruction_full(&bytemuck_id);
    bencher = bencher.bench(("bytemuck::full", &ix, &accounts));

    let (ix, accounts) = instruction_full(&transmute_id);
    bencher = bencher.bench(("transmute::full", &ix, &accounts));

    let (ix, accounts) = instruction_full(&wincode_id);
    bencher = bencher.bench(("wincode::full", &ix, &accounts));

    let (ix, accounts) = instruction_full(&zerocopy_id);
    bencher = bencher.bench(("zerocopy::full", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&bincode_id);
    bencher = bencher.bench(("bincode::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&borsh_id);
    bencher = bencher.bench(("borsh::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&bytemuck_id);
    bencher = bencher.bench(("bytemuck::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&transmute_id);
    bencher = bencher.bench(("transmute::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&wincode_id);
    bencher = bencher.bench(("wincode::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_read_owner(&zerocopy_id);
    bencher = bencher.bench(("zerocopy::read_owner", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&bincode_id);
    bencher = bencher.bench(("bincode::update_amount", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&borsh_id);
    bencher = bencher.bench(("borsh::update_amount", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&bytemuck_id);
    bencher = bencher.bench(("bytemuck::update_amount", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&transmute_id);
    bencher = bencher.bench(("transmute::update_amount", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&wincode_id);
    bencher = bencher.bench(("wincode::update_amount", &ix, &accounts));

    let (ix, accounts) = instruction_update_amount(&zerocopy_id);
    bencher = bencher.bench(("zerocopy::update_amount", &ix, &accounts));

    bencher.execute();
}

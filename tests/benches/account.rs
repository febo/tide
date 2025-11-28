#![feature(test)]

use solana_address::Address;

mod setup;

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_account;
extern crate solana_instruction;
extern crate test;

const DEFAULT_LOADER_KEY: Address = solana_sdk_ids::bpf_loader_upgradeable::ID;

#[cfg(test)]
mod account {

    use crate::setup::{instruction, setup};

    use super::*;
    use mollusk_svm_bencher::MolluskComputeUnitBencher;
    use test::Bencher;

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

        let mut bencher = MolluskComputeUnitBencher::new(mollusk)
            .must_pass(true)
            .out_dir("../target/benches");

        let (ix, accounts) = instruction(&bincode_id);
        bencher = bencher.bench(("bincode::account", &ix, &accounts));

        let (ix, accounts) = instruction(&borsh_id);
        bencher = bencher.bench(("borsh::account", &ix, &accounts));

        let (ix, accounts) = instruction(&bytemuck_id);
        bencher = bencher.bench(("bytemuck::account", &ix, &accounts));

        let (ix, accounts) = instruction(&transmute_id);
        bencher = bencher.bench(("transmute::account", &ix, &accounts));

        let (ix, accounts) = instruction(&wincode_id);
        bencher = bencher.bench(("wincode::account", &ix, &accounts));

        bencher.execute();
    }
}

#![feature(test)]

use solana_address::Address;

mod setup;

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_account;
extern crate solana_instruction;
extern crate test;

const PROGRAM_ID: Address = Address::new_from_array([2; 32]);

#[cfg(test)]
mod wincode {

    use crate::setup::{instruction, setup};

    use super::*;
    use mollusk_svm_bencher::MolluskComputeUnitBencher;
    use test::Bencher;

    #[bench]
    fn run(_bencher: &mut Bencher) {
        let mollusk = setup(&PROGRAM_ID, "transmute_program");
        let mut bencher = MolluskComputeUnitBencher::new(mollusk)
            .must_pass(true)
            .out_dir("../target/benches");

        let (instruction, accounts) = instruction(&PROGRAM_ID);
        bencher = bencher.bench(("transmute::account", &instruction, &accounts));

        bencher.execute();
    }
}

use bincode::{borrow_decode_from_slice, config::standard};
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_address::Address;
use tide::{instruction, setup};
use tide_interface::Account;
use zerocopy::FromBytes;

const PROGRAM_ID: Address = Address::new_from_array([255u8; 32]);

fn run(mollusk: &Mollusk) -> (Address, InstructionResult) {
    let (ix, accounts) = instruction(&PROGRAM_ID);
    let [(account_key, _), ..] = accounts.as_slice() else {
        panic!("expected at least one account");
    };

    let result = mollusk.process_and_validate_instruction(&ix, &accounts, &[Check::success()]);

    let account = result.get_account(account_key);
    assert!(account.is_some());

    (*account_key, result)
}

#[test]
fn test_account_with_bincode() {
    let mollusk = setup(&PROGRAM_ID, "bincode_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let (account, _): (Account, usize) = borrow_decode_from_slice(data, standard().with_fixed_int_encoding()).unwrap();

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

#[test]
fn test_account_with_borsh() {
    let mollusk = setup(&PROGRAM_ID, "borsh_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let mut data = account.unwrap().data.as_slice();
    let account = <Account as borsh::BorshDeserialize>::deserialize(&mut data).unwrap();

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

#[test]
fn test_account_with_bytemuck() {
    let mollusk = setup(&PROGRAM_ID, "bytemuck_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = bytemuck::from_bytes::<Account>(data);

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

#[test]
fn test_account_with_transmute() {
    let mollusk = setup(&PROGRAM_ID, "transmute_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = unsafe { Account::transmute_unchecked(data).unwrap() };

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

#[test]
fn test_account_with_wincode() {
    let mollusk = setup(&PROGRAM_ID, "wincode_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = <Account as wincode::Deserialize>::deserialize(data).unwrap();

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

#[test]
fn test_account_with_zerocopy() {
    let mollusk = setup(&PROGRAM_ID, "zerocopy_program");

    let (key, result) = run(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = Account::ref_from_bytes(data).unwrap();

    assert_eq!(account.state, 255);
    assert_eq!(account.amount, 1_000_000_000);
}

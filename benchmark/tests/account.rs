use bincode::{borrow_decode_from_slice, config::standard};
use mollusk_svm::{
    result::{Check, InstructionResult},
    Mollusk,
};
use solana_address::Address;
use tide::{instruction_full, instruction_read_owner, instruction_update_amount, setup};
use tide_interface::Account;
use zerocopy::FromBytes;

const PROGRAM_ID: Address = Address::new_from_array([255u8; 32]);

fn run(mollusk: &Mollusk) -> (Address, InstructionResult) {
    let (ix, accounts) = instruction_full(&PROGRAM_ID);
    let [(account_key, _), ..] = accounts.as_slice() else {
        panic!("expected at least one account");
    };

    let result = mollusk.process_and_validate_instruction(&ix, &accounts, &[Check::success()]);

    let account = result.get_account(account_key);
    assert!(account.is_some());

    (*account_key, result)
}

fn run_read_owner(mollusk: &Mollusk) -> (Address, InstructionResult) {
    let (ix, accounts) = instruction_read_owner(&PROGRAM_ID);
    let [(account_key, account_data), ..] = accounts.as_slice() else {
        panic!("expected at least one account");
    };

    let result = mollusk.process_and_validate_instruction(&ix, &accounts, &[Check::success()]);

    let account = result.get_account(account_key);
    assert!(account.is_some());

    assert_eq!(account.unwrap().data.len(), account_data.data.len());

    (*account_key, result)
}

fn run_update_amount(mollusk: &Mollusk) -> (Address, InstructionResult) {
    let (ix, accounts) = instruction_update_amount(&PROGRAM_ID);
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

#[test]
fn test_read_owner_with_bincode() {
    let mollusk = setup(&PROGRAM_ID, "bincode_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let (account, _): (Account, usize) = borrow_decode_from_slice(data, standard().with_fixed_int_encoding()).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_read_owner_with_borsh() {
    let mollusk = setup(&PROGRAM_ID, "borsh_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let mut data = account.unwrap().data.as_slice();
    let account = <Account as borsh::BorshDeserialize>::deserialize(&mut data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_read_owner_with_bytemuck() {
    let mollusk = setup(&PROGRAM_ID, "bytemuck_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = bytemuck::from_bytes::<Account>(data);

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_read_owner_with_transmute() {
    let mollusk = setup(&PROGRAM_ID, "transmute_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = unsafe { Account::transmute_unchecked(data).unwrap() };

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_read_owner_with_wincode() {
    let mollusk = setup(&PROGRAM_ID, "wincode_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = <Account as wincode::Deserialize>::deserialize(data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_read_owner_with_zerocopy() {
    let mollusk = setup(&PROGRAM_ID, "zerocopy_program");
    let (key, result) = run_read_owner(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = Account::ref_from_bytes(data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 500_000_000, "amount should remain unchanged");
}

#[test]
fn test_update_amount_with_bincode() {
    let mollusk = setup(&PROGRAM_ID, "bincode_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let (account, _): (Account, usize) = borrow_decode_from_slice(data, standard().with_fixed_int_encoding()).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}

#[test]
fn test_update_amount_with_borsh() {
    let mollusk = setup(&PROGRAM_ID, "borsh_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let mut data = account.unwrap().data.as_slice();
    let account = <Account as borsh::BorshDeserialize>::deserialize(&mut data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}

#[test]
fn test_update_amount_with_bytemuck() {
    let mollusk = setup(&PROGRAM_ID, "bytemuck_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = bytemuck::from_bytes::<Account>(data);

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}

#[test]
fn test_update_amount_with_transmute() {
    let mollusk = setup(&PROGRAM_ID, "transmute_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = account.unwrap().data.as_slice();
    let account = unsafe { Account::transmute_unchecked(data).unwrap() };

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}

#[test]
fn test_update_amount_with_wincode() {
    let mollusk = setup(&PROGRAM_ID, "wincode_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = <Account as wincode::Deserialize>::deserialize(data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}

#[test]
fn test_update_amount_with_zerocopy() {
    let mollusk = setup(&PROGRAM_ID, "zerocopy_program");
    let (key, result) = run_update_amount(&mollusk);

    let account = result.get_account(&key);
    assert!(account.is_some());

    let data = &account.unwrap().data;
    let account = Account::ref_from_bytes(data).unwrap();

    assert_eq!(account.state, 0, "state should remain unchanged");
    assert_eq!(account.amount, 1_000_000_000, "amount should be updated");
}


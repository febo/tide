use std::{slice::from_raw_parts, vec};

use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};

/// System program ID, used for creating accounts.
const SYSTEM_PROGRAM: Address = Address::new_from_array([0; 32]);

/// Base lamports for accounts, used to ensure accounts are rent-exempt.
pub const BASE_LAMPORTS: u64 = 2_000_000_000u64;

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup(program_id: &Address, name: &'static str) -> Mollusk {
    std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    solana_logger::setup();

    Mollusk::new(program_id, name)
}

/// Create an instruction and associated accounts for testing.
///
/// The account is created with only the `owner` field set to a unique address.
#[deprecated(note = "Use `instruction_full` or `instruction_with_type` instead")]
pub fn instruction(program_id: &Address) -> (Instruction, Vec<(Address, Account)>) {
    instruction_with_type(program_id, tide_interface::Instruction::Full)
}

pub fn instruction_with_type(
    program_id: &Address,
    instruction_type: tide_interface::Instruction,
) -> (Instruction, Vec<(Address, Account)>) {
    let account = Address::new_unique();
    let owner = Address::new_unique();

    let mut state = tide_interface::Account::default();
    state.owner = owner.to_bytes();
    state.amount = 500_000_000; // Set initial amount for update tests

    let mut account_data = Account::new(
        BASE_LAMPORTS,
        size_of::<tide_interface::Account>(),
        program_id,
    );
    account_data.data.copy_from_slice(unsafe {
        from_raw_parts(
            &state as *const _ as *const u8,
            size_of::<tide_interface::Account>(),
        )
    });

    let accounts = vec![
        (account, account_data),
        (owner, Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM)),
    ];
    let account_metas = vec![
        AccountMeta::new(account, false),
        AccountMeta::new_readonly(owner, false),
    ];

    (
        Instruction {
            program_id: *program_id,
            accounts: account_metas,
            data: vec![instruction_type as u8],
        },
        accounts,
    )
}

pub fn instruction_full(program_id: &Address) -> (Instruction, Vec<(Address, Account)>) {
    instruction_with_type(program_id, tide_interface::Instruction::Full)
}

pub fn instruction_read_owner(program_id: &Address) -> (Instruction, Vec<(Address, Account)>) {
    instruction_with_type(program_id, tide_interface::Instruction::ReadOwner)
}

pub fn instruction_update_amount(program_id: &Address) -> (Instruction, Vec<(Address, Account)>) {
    instruction_with_type(program_id, tide_interface::Instruction::UpdateAmount)
}

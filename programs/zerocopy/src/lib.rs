#![no_std]

use pinocchio::{
    default_allocator, error::ProgramError, nostd_panic_handler, program_entrypoint, AccountView,
    Address, ProgramResult,
};
use tide_interface::Account;
use zerocopy::FromBytes;

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
default_allocator!();
nostd_panic_handler!();

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [account, owner, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // SAFETY: No other account borrows exist at this point.
    let token_account = Account::mut_from_bytes(unsafe { account.borrow_unchecked_mut() })
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Read something from the account.

    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account.

    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    Ok(())
}

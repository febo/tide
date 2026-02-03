use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
use tide_interface::Account;
use zerocopy::FromBytes;

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

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
    //
    // In general, the safer `try_borrow_mut` method should be used,
    // unless it can be guaranteed that no other borrows exist.
    let account_data = unsafe { account.borrow_unchecked_mut() };

    let token_account =
        Account::mut_from_bytes(account_data).map_err(|_| ProgramError::InvalidAccountData)?;

    // Read something from the account.

    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account.

    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    Ok(())
}

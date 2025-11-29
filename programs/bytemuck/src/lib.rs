use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
use tide_interface::Account;

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
    let token_account =
        bytemuck::try_from_bytes_mut::<Account>(unsafe { account.borrow_unchecked_mut() })
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

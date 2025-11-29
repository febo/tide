use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
use tide_interface::Account;
use wincode::{Deserialize, Serialize};

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

    let mut token_account = {
        // SAFETY: Scoped borrow of the account data.
        let data = unsafe { account.borrow_unchecked() };
        Account::deserialize(data).map_err(|_| ProgramError::InvalidAccountData)?
    };

    // Read something from the account.

    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account.

    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    let mut data = unsafe { account.borrow_unchecked_mut() };
    Account::serialize_into(&mut data, &token_account).map_err(|_| ProgramError::BorshIoError)?;

    Ok(())
}

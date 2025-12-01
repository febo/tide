use bincode_v1::{deserialize, serialize_into};
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

    let mut token_account: Account = {
        // SAFETY: Scoped borrow of the account data.
        let data = unsafe { account.borrow_unchecked() };
        deserialize(data).map_err(|_| ProgramError::InvalidAccountData)?
    };

    // Read something from the account.

    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account.

    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    let data = unsafe { account.borrow_unchecked_mut() };
    serialize_into(data, &token_account).map_err(|_| ProgramError::BorshIoError)?;

    Ok(())
}

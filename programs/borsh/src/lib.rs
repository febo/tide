use borsh::BorshDeserialize;
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

    let mut data = unsafe { account.borrow_unchecked() };
    let account = Account::deserialize(&mut data).map_err(|_| ProgramError::InvalidAccountData)?;

    if &account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    Ok(())
}

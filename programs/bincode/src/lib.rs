use bincode::{borrow_decode_from_slice, encode_into_slice};
use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
use tide_interface::{Account, Instruction};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let [account, owner, ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let instruction = Instruction::try_from_slice(instruction_data)?;

    match instruction {
        Instruction::Full => process_full(account, owner),
        Instruction::ReadOwner => process_read_owner(account, owner),
        Instruction::UpdateAmount => process_update_amount(account),
    }
}

fn process_full(account: &AccountView, owner: &AccountView) -> ProgramResult {
    let config = bincode::config::standard().with_fixed_int_encoding();
    let (mut token_account, _): (Account, usize) = {
        // SAFETY: Scoped borrow of the account data.
        let data = unsafe { account.borrow_unchecked() };
        borrow_decode_from_slice(data, config)
            .map_err(|_| ProgramError::InvalidAccountData)?
    };

    // Read something from the account
    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account
    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    let data = unsafe { account.borrow_unchecked_mut() };
    encode_into_slice(token_account, data, config)
        .map_err(|_| ProgramError::BorshIoError)?;

    Ok(())
}

fn process_read_owner(account: &AccountView, owner: &AccountView) -> ProgramResult {
    let config = bincode::config::standard().with_fixed_int_encoding();
    let (token_account, _): (Account, usize) = {
        let data = unsafe { account.borrow_unchecked() };
        borrow_decode_from_slice(data, config)
            .map_err(|_| ProgramError::InvalidAccountData)?
    };

    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    Ok(())
}

fn process_update_amount(account: &AccountView) -> ProgramResult {
    let config = bincode::config::standard().with_fixed_int_encoding();
    let (mut token_account, _): (Account, usize) = {
        let data = unsafe { account.borrow_unchecked() };
        borrow_decode_from_slice(data, config)
            .map_err(|_| ProgramError::InvalidAccountData)?
    };

    token_account.amount = 1_000_000_000;

    let data = unsafe { account.borrow_unchecked_mut() };
    encode_into_slice(token_account, data, config)
        .map_err(|_| ProgramError::BorshIoError)?;

    Ok(())
}

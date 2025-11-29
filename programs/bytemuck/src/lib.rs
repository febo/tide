use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
use tide_interface::{offsets, Account, Instruction, Pubkey};

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
    // SAFETY: No other account borrows exist at this point.
    let token_account =
        bytemuck::try_from_bytes_mut::<Account>(unsafe { account.borrow_unchecked_mut() })
            .map_err(|_| ProgramError::InvalidAccountData)?;

    // Read something from the account
    if &token_account.owner != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Write something to the account
    token_account.state = 255;
    token_account.amount = 1_000_000_000;

    Ok(())
}

fn process_read_owner(account: &AccountView, owner: &AccountView) -> ProgramResult {
    let data = unsafe { account.borrow_unchecked() };

    let owner_bytes = data
        .get(offsets::OWNER..offsets::OWNER + offsets::OWNER_SIZE)
        .ok_or(ProgramError::InvalidAccountData)?;

    let owner_field = bytemuck::try_from_bytes::<Pubkey>(owner_bytes)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if owner_field != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    Ok(())
}

fn process_update_amount(account: &AccountView) -> ProgramResult {
    let data = unsafe { account.borrow_unchecked_mut() };

    let amount_bytes = data
        .get_mut(offsets::AMOUNT..offsets::AMOUNT + offsets::AMOUNT_SIZE)
        .ok_or(ProgramError::InvalidAccountData)?;

    let amount_field = bytemuck::try_from_bytes_mut::<u64>(amount_bytes)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    *amount_field = 1_000_000_000;

    Ok(())
}

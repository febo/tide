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
    let token_account =
        unsafe { Account::transmute_unchecked_mut(account.borrow_unchecked_mut())? };

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

    if data.len() < offsets::OWNER + offsets::OWNER_SIZE {
        return Err(ProgramError::InvalidAccountData);
    }

    let owner_field = unsafe { &*(data.as_ptr().add(offsets::OWNER) as *const Pubkey) };

    if owner_field != owner.address().as_array() {
        return Err(ProgramError::IncorrectAuthority);
    }

    Ok(())
}

fn process_update_amount(account: &AccountView) -> ProgramResult {
    let data = unsafe { account.borrow_unchecked_mut() };

    if data.len() < offsets::AMOUNT + offsets::AMOUNT_SIZE {
        return Err(ProgramError::InvalidAccountData);
    }
    
    let amount_field = unsafe { &mut *(data.as_mut_ptr().add(offsets::AMOUNT) as *mut u64) };

    *amount_field = 1_000_000_000;

    Ok(())
}

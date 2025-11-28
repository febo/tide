#![no_std]

#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use solana_program_error::ProgramError;
#[cfg(feature = "wincode")]
use wincode::{SchemaRead, SchemaWrite};

/// A public key (32 bytes).
pub type Pubkey = [u8; 32];

#[repr(C)]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize))]
#[cfg_attr(feature = "bytemuck", derive(Copy, Clone, Pod, Zeroable))]
#[cfg_attr(feature = "wincode", derive(SchemaWrite, SchemaRead))]
#[derive(Debug, Default)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,

    /// The owner of this account.
    pub owner: Pubkey,

    /// The amount of tokens this account holds.
    amount: [u8; 8],

    /// If `delegate` is `Some` then `delegated_amount` represents
    /// the amount authorized by the delegate.
    delegate_option: [u8; 4],

    delegate: Pubkey,

    /// The account's state.
    state: u8,

    /// Indicates whether this account represents a native token or not.
    is_native: [u8; 4],

    /// If `is_native.is_some`, this is a native token, and the value logs the
    /// rent-exempt reserve. An Account is required to be rent-exempt, so
    /// the value is used by the Processor to ensure that wrapped SOL
    /// accounts do not drop below this threshold.
    native_amount: [u8; 8],

    /// The amount delegated.
    delegated_amount: [u8; 8],

    /// Optional authority to close the account.
    close_authority_option: [u8; 4],

    close_authority: Pubkey,
}

impl Account {
    /// Transmute a byte slice into an `Account` reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` is a valid representation of `Account`.
    #[inline(always)]
    pub unsafe fn transmute_unchecked(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&*(bytes.as_ptr() as *const Self))
    }
}

#![no_std]

#[cfg(feature = "bincode-v2")]
use bincode::{Decode, Encode};
#[cfg(feature = "borsh")]
use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "bincode-v1")]
use serde::{Deserialize, Serialize};
use solana_program_error::ProgramError;
#[cfg(feature = "wincode")]
use wincode::{SchemaRead, SchemaWrite};
#[cfg(feature = "zerocopy")]
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// A public key (32 bytes).
pub type Pubkey = [u8; 32];

/// Account data structure for a token account.
///
/// This is a simplified version of the SPL Token account structure
/// to demonstrate the concept.
#[repr(C)]
#[cfg_attr(feature = "bincode-v1", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "bincode-v2", derive(Decode, Encode))]
#[cfg_attr(feature = "borsh", derive(BorshDeserialize, BorshSerialize))]
#[cfg_attr(feature = "bytemuck", derive(Copy, Clone, Pod, Zeroable))]
#[cfg_attr(feature = "wincode", derive(SchemaWrite, SchemaRead))]
#[cfg_attr(
    feature = "zerocopy",
    derive(KnownLayout, FromBytes, Immutable, IntoBytes)
)]
#[derive(Debug, Default)]
pub struct Account {
    /// The mint associated with this account
    pub mint: Pubkey,

    /// The owner of this account.
    pub owner: Pubkey,

    /// The amount of tokens this account holds.
    pub amount: u64,

    /// The delegate for this account.
    pub delegate: Pubkey,

    /// The account's state.
    pub state: u8,

    /// Padding bytes.
    _padding: [u8; 7],

    /// Native token amount.
    pub native_amount: u64,

    /// The amount delegated.
    pub delegated_amount: u64,

    /// The close authority.
    pub close_authority: Pubkey,
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

    /// Transmute a mutable byte slice into a mutable `Account` reference.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` is a valid representation of `Account`.
    #[inline(always)]
    pub unsafe fn transmute_unchecked_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != size_of::<Self>() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&mut *(bytes.as_mut_ptr() as *mut Self))
    }
}

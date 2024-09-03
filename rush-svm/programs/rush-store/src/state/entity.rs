use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// OPT-OUT: didn't use #[seeds()] because ShankAccount seeds
// helper attribute is buggy. PDA is generated offchain
// instead and seeds are validated

#[repr(C)]
#[rustfmt::skip]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, Eq, PartialEq, ShankAccount, SplDiscriminate)]
#[discriminator_hash_input("rush_store::state::entity")]
pub struct Entity {
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],
    /// Canonical bump for Entity 
    pub bump: u8,

    /// Padding to remove SLOP in C memory layout alignment
    /// Widest scalar = 8bytes
    _padding: [u8; 7]
}

impl Entity {
    /// Is `true` if Entity is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == Entity::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if Entity is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

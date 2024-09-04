use borsh::{BorshDeserialize, BorshSerialize};
use rush_core::blueprint::{Component, ComponentValue};
use shank::ShankAccount;
use solana_program::{hash::Hash, pubkey::Pubkey};
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};
use std::collections::BTreeMap;

// OPT-OUT: didn't use #[seeds()] because ShankAccount seeds
// helper attribute is buggy. PDA is generated offchain
// instead and seeds are validated

#[derive(
    Clone,
    BorshSerialize,
    BorshDeserialize,
    Debug,
    Default,
    Eq,
    PartialEq,
    ShankAccount,
    SplDiscriminate,
)]
#[discriminator_hash_input("rush_store::state::Instance")]
pub struct Instance {
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],

    /// Source of truth for what the values
    pub components: BTreeMap<Component, ComponentValue>,
    /// Nonce to allow multiple Instances
    pub nonce: u64,
    /// Overaching World authority who has access to state changing
    /// operations in the World
    pub world_authority: Pubkey,
    /// Overaching Region authority who has access to state changing
    /// operations in the Region
    pub region_authority: Pubkey,
    /// Overaching Instance authority who has access to state changing
    /// operations in this specific Instance
    pub instance_authoriy: Pubkey,

    /// Canonical bump for Instances
    pub bump: u8,
}

impl Instance {
    /// Is `true` if Instances is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == Instance::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if Instance is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

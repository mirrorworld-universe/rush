use super::{Entity, Region};
use borsh::{BorshDeserialize, BorshSerialize};
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
#[discriminator_hash_input("rush_store::state::World")]
pub struct World {
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],

    /// Description of the world
    pub name: String,
    /// Description of the world
    pub description: String,
    /// Onchain record of what Entity types exist in the world
    pub entities: Vec<Entity>,
    /// Onchain record of what Regions exist in the world
    pub regions: Vec<Region>,
    /// Source of truth for what Instances exist in the world
    pub instances: BTreeMap<Hash, u64>,
    /// Overaching authority who has access to state changing
    /// operations
    pub world_auhtority: Pubkey,

    /// Canonical bump for World
    pub bump: u8,
}

impl World {
    /// Is `true` if World is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == World::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if World is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

use borsh::{BorshDeserialize, BorshSerialize};
use rush_core::blueprint::{Entity, Region};
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
    pub instances: BTreeMap<Region, BTreeMap<Entity, u64>>,
    /// Determines if the World is already launched and
    /// instances can now be Created, Updated, and Deleted
    /// outside of the CreateWorld (Initialization) Instruction
    pub is_launched: bool,
    /// Overaching authority who has access to state changing
    /// operations
    pub world_authority: Pubkey,

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

    /// Create new World state
    pub fn new(
        name: String,
        description: String,
        world_authority: Pubkey,
        regions: Vec<Region>,
        entities: Vec<Entity>,
        bump: u8,
    ) -> Self {
        Self {
            name,
            description,
            world_authority,
            regions,
            entities,
            bump,
            discriminator: World::SPL_DISCRIMINATOR.into(),
            instances: BTreeMap::new(),
            is_launched: false,
        }
    }
}

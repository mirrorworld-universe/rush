use borsh::{BorshDeserialize, BorshSerialize};
use rush_ecs_core::blueprint::{Entity, Region};
use shank::ShankAccount;
use solana_program::{msg, pubkey::Pubkey};
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
#[discriminator_hash_input("rush_ecs_store::state::World")]
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
    ///
    /// # Arguments
    ///
    /// * `name` - A [`String`] that holds the name of the world
    /// * `description` - A [`String`] that holds the description of the world
    /// * `world_authority` - A [`Pubkey`] of the authority who can change the world's account data
    /// * `regions` - A Vector of the new type [`Region`] holding all the existing regions in the world
    /// * `entities` - A Vector of the new type [`Entity`] holding all the existing entities in the world
    /// * `bump` - A [`u8`] holding the canonical bump of the World's onchain account used for PDA derivation
    /// * `preload` - A [`bool`] that allows preloading of [`BTreeMap`] keys in the [`World`] `instances` property
    ///
    pub fn new(
        name: String,
        description: String,
        world_authority: Pubkey,
        regions: Vec<Region>,
        entities: Vec<Entity>,
        bump: u8,
        preload: bool,
    ) -> Self {
        let mut instances = BTreeMap::new();

        // preload regions and entities in instances BTreeMap
        if preload {
            for r in regions.iter() {
                instances.insert(r.clone(), BTreeMap::new());
                // unwrap is ok, None case already checked
                let curr_region_mut = instances.get_mut(r).unwrap();

                for e in entities.iter() {
                    curr_region_mut.insert(e.clone(), u64::MIN);
                }
            }
        }

        Self {
            name,
            description,
            world_authority,
            regions,
            entities,
            bump,
            discriminator: World::SPL_DISCRIMINATOR.into(),
            is_launched: false,
            instances,
        }
    }
}

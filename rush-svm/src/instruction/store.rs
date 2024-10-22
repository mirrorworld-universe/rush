use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use rush_core::blueprint::{Component, ComponentValue, Entity, Region};
use shank::{ShankContext, ShankInstruction};

/// RushStore Instruction List
///
/// For World Authority:
/// - CreateWorld, UpdateWorld, DeleteWorld
///
/// For Region Authority:
/// - UpdateEntity, DespawnEntity
///
/// For Entity Authority:
/// - SpawnEntity, UpdateEntity, DespawnEntity
///
#[derive(
    BorshDeserialize, BorshSerialize, Clone, Debug, Eq, PartialEq, ShankContext, ShankInstruction,
)]
pub enum RushStoreInstruction {
    #[account(
        0,
        name = "world_authority",
        desc = "World authority who has access to World state changing operations"
    )]
    #[account(
        1,
        signer,
        name = "payer",
        desc = "Payer who funds the state account creation"
    )]
    #[account(2, writable, name = "world", desc = "World State PDA")]
    #[account(3, name = "system_program", desc = "System Program")]
    CreateWorld {
        name: String,
        description: String,
        regions: Vec<Region>,
        entities: Vec<Entity>,
        bump: u8,
    },

    #[account(
        0,
        signer,
        name = "world_authority",
        desc = "World authority who has access to World state changing operations"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    #[account(2, name = "system_program", desc = "System Program")]
    UpdateWorld {
        regions: Vec<Region>,
        entities: Vec<Entity>,
    },

    #[account(
        0,
        signer,
        name = "world_authority",
        desc = "World authority who has access to World state changing operations"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    DeleteWorld,

    #[account(
        0,
        signer,
        name = "instance_authority",
        desc = "Instance authority who has access to Instance state changing operations"
    )]
    #[account(1, writable, name = "instance", desc = "Instance State PDA")]
    #[account(2, writable, name = "world", desc = "World State PDA")]
    #[account(3, name = "system_program", desc = "System Program")]
    SpawnEntity {
        region: Region,
        entity: Entity,
        components: BTreeMap<Component, ComponentValue>,
        nonce: u64,
        bump: u8,
    },

    #[account(
        0,
        signer,
        name = "instance_authority",
        desc = "Instance authority who has access to Instance state changing operations"
    )]
    #[account(1, writable, name = "instance", desc = "Instance State PDA")]
    UpdateEntity {
        component: Component,
        value: ComponentValue,
    },

    #[account(
        0,
        signer,
        name = "instance_authority",
        desc = "Instance authority who has access to Instance state changing operations"
    )]
    #[account(1, writable, name = "instance", desc = "Instance State PDA")]
    DespawnEntity,
}

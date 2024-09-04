use borsh::{BorshDeserialize, BorshSerialize};
use rush_core::blueprint::{Entity, Region};
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
        signer,
        name = "world_authority",
        desc = "World authority who has access to"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    #[account(2, name = "system_program", desc = "System Program")]
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
        desc = "World authority who has access to"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    UpdateWorld {
        regions: Vec<Region>,
        entities: Vec<Entity>,
    },

    #[account(
        0,
        signer,
        name = "world_authority",
        desc = "World authority who has access to"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    DeleteWorld,

    #[account(0, writable, name = "payer", desc = "Account description")]
    SpawnEntity,

    #[account(0, writable, name = "payer", desc = "Account description")]
    UpdateEntity,

    #[account(0, writable, name = "payer", desc = "Account description")]
    DespawnEntity,
}

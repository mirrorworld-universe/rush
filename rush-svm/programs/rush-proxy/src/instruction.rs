use borsh::{BorshDeserialize, BorshSerialize};
use rush_core::blueprint::{Entity, Region};
use shank::{ShankContext, ShankInstruction};

#[derive(
    BorshDeserialize, BorshSerialize, Clone, Debug, Eq, PartialEq, ShankContext, ShankInstruction,
)]
pub enum RushProxyInstruction {
    #[account(
        0,
        signer,
        name = "user_authority",
        desc = "User authority who has access to User state changing operations"
    )]
    #[account(1, writable, name = "user", desc = "User State PDA")]
    #[account(2, name = "world", desc = "World State PDA")]
    #[account(3, name = "system_program", desc = "System Program")]
    Register { user_agent_salt: String, bump: u8 },

    #[account(
        0,
        signer,
        name = "user_authority",
        desc = "User authority who has access to User state changing operations"
    )]
    #[account(1, writable, name = "user", desc = "User State PDA")]
    Deregister,

    #[account(
        0,
        signer,
        name = "world_authority",
        desc = "World authority who has access to World state changing operations"
    )]
    #[account(1, writable, name = "world", desc = "World State PDA")]
    #[account(2, name = "rush_store_program", desc = "Rush Store Program")]
    #[account(3, name = "system_program", desc = "System Program")]
    ProxyCreateWorld {
        user_agent_salt: String,
        user_bump: u8,
        name: String,
        description: String,
        regions: Vec<Region>,
        entities: Vec<Entity>,
        world_bump: u8,
    },
}

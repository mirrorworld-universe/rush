use crate::instruction::RushProxyInstruction;
use rush_core::blueprint::{Component, ComponentValue, Entity, Region};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

#[allow(clippy::too_many_arguments)]
pub fn ix_register(
    program_id: &Pubkey,
    user_agent_salt: String,
    bump: u8,
    user: &Pubkey,
    world: &Pubkey,
    user_authority: &Pubkey,
) -> Instruction {
    let instruction = RushProxyInstruction::Register {
        user_agent_salt,
        bump,
    };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        // Accounts
        // 0. `[SIGNER]`       User Authority
        // 1. `[WRITE]`        User State PDA
        // 2. `[]`             World State PDA
        // 3. `[]`             System Program
        vec![
            AccountMeta::new(*user_authority, true),
            AccountMeta::new(*user, false),
            AccountMeta::new_readonly(*world, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    )
}

#[allow(clippy::too_many_arguments)]
pub fn ix_proxy_create_world(
    program_id: &Pubkey,
    user_agent_salt: String,
    user_bump: u8,
    name: String,
    description: String,
    regions: Vec<Region>,
    entities: Vec<Entity>,
    world_bump: u8,
    user_authority: &Pubkey,
    user: &Pubkey,
    world: &Pubkey,
    rush_store_program_id: &Pubkey,
) -> Instruction {
    let instruction = RushProxyInstruction::ProxyCreateWorld {
        user_agent_salt,
        user_bump,
        name,
        description,
        regions,
        entities,
        world_bump,
    };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new_readonly(*user, false),
            AccountMeta::new(*user_authority, true),
            AccountMeta::new(*world, false),
            AccountMeta::new_readonly(*rush_store_program_id, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    )
}

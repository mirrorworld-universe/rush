use crate::instruction::RushStoreInstruction;
use rush_core::blueprint::{Entity, Region};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};

#[allow(clippy::too_many_arguments)]
pub fn ix_create_world(
    program_id: &Pubkey,
    name: String,
    description: String,
    regions: Vec<Region>,
    entities: Vec<Entity>,
    bump: u8,
    world: &Pubkey,
    world_authority: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::CreateWorld {
        name,
        description,
        regions,
        entities,
        bump,
    };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*world_authority, true),
            AccountMeta::new(*world, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    )
}

pub fn ix_update_world(
    program_id: &Pubkey,
    regions: Vec<Region>,
    entities: Vec<Entity>,
    world: &Pubkey,
    world_authority: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::UpdateWorld { regions, entities };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*world_authority, true),
            AccountMeta::new(*world, false),
        ],
    )
}

pub fn ix_delete_world(
    program_id: &Pubkey,
    world: &Pubkey,
    world_authority: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::DeleteWorld;

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*world_authority, true),
            AccountMeta::new(*world, false),
        ],
    )
}

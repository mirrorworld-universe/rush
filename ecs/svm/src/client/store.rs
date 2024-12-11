use crate::instruction::RushStoreInstruction;
use rush_ecs_core::blueprint::{Component, ComponentValue, Entity, Region};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program::ID as SYSTEM_PROGRAM_ID,
};
use std::collections::BTreeMap;

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
    payer: &Pubkey,
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
            AccountMeta::new_readonly(*world_authority, false),
            AccountMeta::new(*payer, true),
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
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
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

#[allow(clippy::too_many_arguments)]
pub fn ix_spawn_entity(
    program_id: &Pubkey,
    region: Region,
    entity: Entity,
    components: BTreeMap<Component, ComponentValue>,
    nonce: u64,
    bump: u8,
    instance: &Pubkey,
    instance_authority: &Pubkey,
    world: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::SpawnEntity {
        region,
        entity,
        components,
        nonce,
        bump,
    };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*instance_authority, true),
            AccountMeta::new(*instance, false),
            AccountMeta::new(*world, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    )
}

pub fn ix_update_entity(
    program_id: &Pubkey,
    component: Component,
    value: ComponentValue,
    instance: &Pubkey,
    instance_authority: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::UpdateEntity { component, value };

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*instance_authority, true),
            AccountMeta::new(*instance, false),
        ],
    )
}

pub fn ix_despawn_entity(
    program_id: &Pubkey,
    instance: &Pubkey,
    instance_authority: &Pubkey,
) -> Instruction {
    let instruction = RushStoreInstruction::DeleteWorld;

    Instruction::new_with_borsh(
        *program_id,
        &instruction,
        vec![
            AccountMeta::new(*instance_authority, true),
            AccountMeta::new(*instance, false),
        ],
    )
}

pub mod create_world;
pub mod delete_world;
pub mod despawn_entity;
pub mod spawn_entity;
pub mod update_entity;
pub mod update_world;

use create_world::*;
use delete_world::*;
use despawn_entity::*;
use spawn_entity::*;
use update_entity::*;
use update_world::*;

use borsh::BorshDeserialize;
use rush_ecs_svm::instruction::{accounts::*, RushStoreInstruction};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub struct Processor {}

impl<'a> Processor {
    /// Process the transaction
    ///
    /// - Deserializes the instruction data
    /// - Routes transaction data to the proper handler
    pub fn process(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // get instruction
        let instruction = RushStoreInstruction::try_from_slice(instruction_data)?;
        match instruction {
            RushStoreInstruction::CreateWorld {
                name,
                description,
                regions,
                entities,
                bump,
            } => process_create_world(
                program_id,
                CreateWorldAccounts::context(accounts)?,
                name,
                description,
                regions,
                entities,
                bump,
            )?,

            RushStoreInstruction::UpdateWorld { regions, entities } => process_update_world(
                program_id,
                UpdateWorldAccounts::context(accounts)?,
                regions,
                entities,
            )?,

            RushStoreInstruction::DeleteWorld => {
                process_delete_world(program_id, DeleteWorldAccounts::context(accounts)?)?
            }

            RushStoreInstruction::SpawnEntity {
                region,
                entity,
                components,
                nonce,
                bump,
            } => process_spawn_entity(
                program_id,
                SpawnEntityAccounts::context(accounts)?,
                region,
                entity,
                components,
                nonce,
                bump,
            )?,

            RushStoreInstruction::UpdateEntity { component, value } => process_update_entity(
                program_id,
                UpdateEntityAccounts::context(accounts)?,
                component,
                value,
            )?,

            RushStoreInstruction::DespawnEntity => {
                process_despawn_entity(program_id, DespawnEntityAccounts::context(accounts)?)?
            }
        }

        Ok(())
    }
}

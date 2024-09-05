use borsh::BorshSerialize;
use rush_core::blueprint::{Entity, Region};
use rush_svm::{
    instruction::accounts::{Context, UpdateWorldAccounts},
    state::World,
};
use solana_program::{borsh1, entrypoint::ProgramResult, pubkey::Pubkey};

/// Update World
///
/// - Try data slice with Borsh1 Unchecked
/// - Update World State PDA with new entities / regions
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
///
/// Instruction Data
/// - regions: Vec<Region>,
/// - entities: Vec<Entity>
///
/// Data Validations
/// -
///
pub fn process_update_world(
    _program_id: &Pubkey,
    ctx: Context<UpdateWorldAccounts>,
    regions: Vec<Region>,
    entities: Vec<Entity>,
) -> ProgramResult {
    let mut world_data = ctx.accounts.world.try_borrow_mut_data()?;

    // need to use Borsh version 1 for dynamic data
    // else, de/serialization will fail with Account Unknown Error at runtime
    let mut world = borsh1::try_from_slice_unchecked::<World>(&world_data)?;

    // update world state
    world.regions = regions;
    world.entities = entities;

    // store new world state
    world.serialize(&mut &mut world_data[..])?;

    Ok(())
}

use borsh::BorshSerialize;
use rush_core::blueprint::{Entity, Region};
use rush_svm::instruction::accounts::{Context, UpdateWorldAccounts};
use rush_svm::state::World;
use solana_program::{
    borsh1, entrypoint::ProgramResult, program::invoke, pubkey::Pubkey, system_instruction,
    sysvar::rent::Rent, sysvar::Sysvar,
};

/// Update World
///
/// - Try data slice with Borsh1 Unchecked
/// - Update World State PDA with new entities / regions
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
/// 2. `[]`             System Program
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
    let world_data = ctx.accounts.world.try_borrow_data()?;

    // need to use Borsh version 1 for dynamic data
    // else, de/serialization will fail with Account Unknown Error at runtime
    let mut world = borsh1::try_from_slice_unchecked::<World>(&world_data)?;
    // Relinquish borrow for realloc
    drop(world_data);

    // update world state
    world.regions = regions;
    world.entities = entities;

    // realloc
    let new_world_size = borsh1::get_instance_packed_len::<World>(&world)?;
    let new_rent = Rent::get()?.minimum_balance(new_world_size);
    let new_lamports_needed = new_rent.saturating_sub(ctx.accounts.world.lamports());

    ctx.accounts.world.realloc(new_world_size, true)?;

    // make rent-exempt again after resize
    let from = ctx.accounts.world_authority.key;
    let to = ctx.accounts.world.key;
    let ix = system_instruction::transfer(from, to, new_lamports_needed);

    invoke(
        &ix,
        &[
            ctx.accounts.world_authority.clone(),
            ctx.accounts.world.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    // store new world state
    let mut world_data = ctx.accounts.world.try_borrow_mut_data()?;
    world.serialize(&mut &mut world_data[..])?;

    Ok(())
}

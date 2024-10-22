use borsh::BorshSerialize;
use rush_core::blueprint::{Entity, Region};
use rush_svm::instruction::accounts::{Context, CreateWorldAccounts};
use rush_svm::{pda::WorldPDA, state::World};
use solana_program::{
    borsh1, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, rent::Rent,
    system_instruction, sysvar::Sysvar,
};

/// Create World
///
/// - Creates a new account for World state
/// - Stores initial World state in newly created account
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
/// 2. `[]`             System Program
///
/// Instruction Data
/// - name: String
/// - description: String
/// - regions: Vec<Region>,
/// - entities: Vec<Entity>,
/// - bump: u8
///
/// Data Validations
/// -
///
pub fn process_create_world(
    program_id: &Pubkey,
    ctx: Context<CreateWorldAccounts>,
    name: String,
    description: String,
    regions: Vec<Region>,
    entities: Vec<Entity>,
    bump: u8,
) -> ProgramResult {
    let new_world_state = World::new(
        name.clone(),
        description.clone(),
        *ctx.accounts.world_authority.key,
        regions.clone(),
        entities.clone(),
        bump,
        true,
    );

    // need to use Borsh version 1 for dynamic data
    // else, de/serialization will fail with Account Unknown Error at runtime
    let new_world_size = borsh1::get_instance_packed_len(&new_world_state)?;

    // rent from dynamic data size
    let rent_exempt_cost = Rent::get()?.minimum_balance(new_world_size);
    // space from dynamic data size
    let space_needed: u64 = new_world_size as u64;

    // build create_account instruction
    let create_world_account_ix = system_instruction::create_account(
        ctx.accounts.payer.key,
        ctx.accounts.world.key,
        rent_exempt_cost,
        space_needed,
        program_id,
    );

    // invoke CPI instruction
    invoke_signed(
        &create_world_account_ix,
        &[ctx.accounts.payer.clone(), ctx.accounts.world.clone()],
        &[&[
            WorldPDA::TAG.as_bytes(),
            name.as_bytes(),
            description.as_bytes(),
            &[bump],
        ]],
    )?;

    // store new World state into newly created account
    let mut new_world_raw_bytes = ctx.accounts.world.try_borrow_mut_data()?;
    new_world_state.serialize(&mut &mut new_world_raw_bytes[..])?;

    Ok(())
}

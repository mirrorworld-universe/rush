use crate::cpi::ix_create_world;
use crate::instruction::accounts::{Context, ProxyCreateWorldAccounts};
use borsh::BorshSerialize;
use rush_core::blueprint::{Entity, Region};
use rush_svm::{
    instruction::RushStoreInstruction,
    pda::{UserPDA, WorldPDA},
    state::World,
};
use solana_program::{
    borsh1,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
///
/// - Proxies the CreateWorld instruction to create
/// custom behavior on top of main storage logic
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
/// 2. `[]`             Rush Store Program
/// 3. `[]`             System Program
///
/// Instruction Data
/// - user_agent_salt: String,
/// - user_bump: u8,
/// - name: String,
/// - description: String,
/// - regions: Vec<Region>,
/// - entities: Vec<Entity>,
/// - world_bump: u8,
///
/// Data Validations
/// -
///
#[allow(clippy::too_many_arguments)]
pub fn process_proxy_create_world(
    _program_id: &Pubkey,
    ctx: Context<ProxyCreateWorldAccounts>,
    user_agent_salt: String,
    user_bump: u8,
    name: String,
    description: String,
    regions: Vec<Region>,
    entities: Vec<Entity>,
    world_bump: u8,
) -> ProgramResult {
    let ix = ix_create_world(
        ctx.accounts.rush_store_program.key,
        name.clone(),
        description.clone(),
        regions,
        entities,
        world_bump,
        ctx.accounts.world.key,
        ctx.accounts.world_authority.key,
    );

    // invoke CPI instruction
    invoke_signed(
        &ix,
        &[
            ctx.accounts.world_authority.clone(),
            ctx.accounts.world.clone(),
            ctx.accounts.system_program.clone(),
        ],
        &[&[
            UserPDA::TAG.as_bytes(),
            ctx.accounts.world.key.as_ref(),
            ctx.accounts.world_authority.key.as_ref(),
            user_agent_salt.as_bytes(),
            &[user_bump],
        ]],
    )?;

    Ok(())
}

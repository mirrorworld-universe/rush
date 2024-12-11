use crate::instruction::accounts::{Context, ProxyCreateWorldAccounts};
use crate::store_cpi;
use borsh::BorshSerialize;
use rush_ecs_core::blueprint::{Entity, Region};
use rush_ecs_svm::{
    instruction::RushStoreInstruction,
    pda::{UserPDA, WorldPDA},
    state::World,
};
use solana_program::{
    borsh1,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
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
    // let ix = store_cpi::ix_create_world(
    //     ctx.accounts.rush_store_program.key,
    //     name.clone(),
    //     description.clone(),
    //     regions,
    //     entities,
    //     world_bump, ctx.accounts.world.key,
    //     ctx.accounts.user.key,
    // );

    let instruction = RushStoreInstruction::CreateWorld {
        name,
        description,
        regions,
        entities,
        bump: world_bump,
    };

    let ix = Instruction::new_with_borsh(
        *ctx.accounts.rush_store_program.key,
        &instruction,
        vec![
            AccountMeta::new_readonly(*ctx.accounts.user.key, false),
            AccountMeta::new(*ctx.accounts.user_authority.key, true),
            AccountMeta::new(*ctx.accounts.world.key, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    );

    msg!(
        "SIGNER SEEDS: {}, {}, {}, {}, {}",
        UserPDA::TAG,
        ctx.accounts.world.key,
        ctx.accounts.user_authority.key,
        user_agent_salt,
        user_bump
    );

    msg!(
        "LENGTH of SIGNER SEEDS: {}, {}, {}, {}, {}",
        UserPDA::TAG,
        ctx.accounts.world.key,
        ctx.accounts.user_authority.key,
        user_agent_salt,
        user_bump
    );

    let created_user_pda = UserPDA::create_pda(
        _program_id,
        ctx.accounts.user_authority.key,
        ctx.accounts.world.key,
        user_agent_salt.clone(),
        user_bump,
    );

    msg!("PROXY PROGRAM ID INSIDE: {}", _program_id);
    msg!("USER ACCOUNT INFO: {:?}", *ctx.accounts.user);
    msg!("USER PDA INSIDE: {}", *ctx.accounts.user.key);
    msg!("CREATED USER PDA INSIDE: {}", created_user_pda);
    msg!("USER BUMP: {}", user_bump);

    // invoke CPI instruction
    invoke_signed(
        &ix,
        &[
            ctx.accounts.user.clone(),
            ctx.accounts.user_authority.clone(),
            ctx.accounts.world.clone(),
            ctx.accounts.system_program.clone(),
        ],
        &[&[
            UserPDA::TAG.as_bytes(),
            ctx.accounts.world.key.as_ref(),
            ctx.accounts.user_authority.key.as_ref(),
            user_agent_salt.as_bytes(),
            &[user_bump],
        ]],
    )?;

    Ok(())
}

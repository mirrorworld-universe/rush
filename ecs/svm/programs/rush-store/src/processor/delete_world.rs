use rush_ecs_svm::instruction::accounts::{Context, DeleteWorldAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Delete World
///
/// - Transfers all lamports from World State PDA
/// to World Authority (Signer) for closing
/// - Fills data with 0s for closing
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
///
/// Instruction Data
/// - (None)
///
/// Data Validations
/// -
///
pub fn process_delete_world(
    _program_id: &Pubkey,
    ctx: Context<DeleteWorldAccounts>,
) -> ProgramResult {
    // World State PDA
    let world_account_lamports = ctx.accounts.world.lamports();

    // World Authority
    let world_authority_lamports = ctx.accounts.world_authority.lamports();

    // direct transfer token_base (PDA) lamports into sale_authority
    // NOTE: Direct transfer is okay since token_base is a PDA owned by sale_authority
    //
    // I know, it looks like it's illegal but Solana ALWAYS does balances
    // and checks pre and post so this is totally safe
    **ctx.accounts.world_authority.try_borrow_mut_lamports()? = world_authority_lamports
        .checked_add(world_account_lamports) // None if overflow
        .unwrap();

    // zero out World State PDA lamports
    **ctx.accounts.world.try_borrow_mut_lamports()? = 0;

    // fill World State PDA with 0s = no data
    let mut world_data = ctx.accounts.world.try_borrow_mut_data()?;
    world_data.fill(0);

    Ok(())
}

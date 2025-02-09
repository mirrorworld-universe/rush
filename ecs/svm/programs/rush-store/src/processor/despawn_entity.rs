use rush_ecs_svm::instruction::accounts::{Context, DespawnEntityAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Despawn Entity
///
/// - Transfers all lamports from Instance State PDA
/// to Instance Authority (Signer) for closing
/// - Fills data with 0s for closing
///
/// Accounts
/// 0. `[SIGNER]`       Instance Authority
/// 1. `[WRITE]`        Instance PDA
///
/// Instruction Data
/// - (None)
///
/// Data Validations
/// -
///
pub fn process_despawn_entity(
    _program_id: &Pubkey,
    ctx: Context<DespawnEntityAccounts>,
) -> ProgramResult {
    // Instance State PDA
    let instance_account_lamports = ctx.accounts.instance.lamports();

    // Instance Authority
    let instance_authority_lamports = ctx.accounts.instance_authority.lamports();

    // direct transfer token_base (PDA) lamports into sale_authority
    // NOTE: Direct transfer is okay since token_base is a PDA owned by sale_authority
    //
    // I know, it looks like it's illegal but Solana ALWAYS does balances
    // and checks pre and post so this is totally safe
    **ctx.accounts.instance_authority.try_borrow_mut_lamports()? = instance_authority_lamports
        .checked_add(instance_account_lamports) // None if overflow
        .unwrap();

    // zero out Instance State PDA lamports
    **ctx.accounts.instance.try_borrow_mut_lamports()? = 0;

    // fill Instance State PDA with 0s = no data
    let mut instance_data = ctx.accounts.instance.try_borrow_mut_data()?;
    instance_data.fill(0);

    Ok(())
}

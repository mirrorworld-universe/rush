use rush_svm::instruction::accounts::{Context, DespawnEntityAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Despawn Entity
///
/// - Side effect #1
/// - Side effect #2
///
/// Accounts
/// 0. `[]`    Description
///
/// Instruction Data
/// -
///
/// Data Validations
/// -
pub fn process_despawn_entity(
    program_id: &Pubkey,
    ctx: Context<DespawnEntityAccounts>,
) -> ProgramResult {
    Ok(())
}

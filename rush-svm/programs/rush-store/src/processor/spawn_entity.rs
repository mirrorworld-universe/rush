use crate::instruction::accounts::{Context, SpawnEntityAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Spawn Entity
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
pub fn process_spawn_entity(
    program_id: &Pubkey,
    ctx: Context<SpawnEntityAccounts>,
) -> ProgramResult {
    Ok(())
}

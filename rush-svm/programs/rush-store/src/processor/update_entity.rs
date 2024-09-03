use crate::instruction::accounts::{Context, UpdateEntityAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Update Entity
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
pub fn process_update_entity(
    program_id: &Pubkey,
    ctx: Context<UpdateEntityAccounts>,
) -> ProgramResult {
    Ok(())
}

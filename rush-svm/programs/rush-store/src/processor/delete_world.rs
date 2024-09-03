use crate::instruction::accounts::{Context, DeleteWorldAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Delete World
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
pub fn process_delete_world(
    program_id: &Pubkey,
    ctx: Context<DeleteWorldAccounts>,
) -> ProgramResult {
    Ok(())
}

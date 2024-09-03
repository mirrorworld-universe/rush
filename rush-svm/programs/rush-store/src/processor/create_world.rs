use crate::instruction::accounts::{Context, CreateWorldAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Create World
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
pub fn process_create_world(
    program_id: &Pubkey,
    ctx: Context<CreateWorldAccounts>,
) -> ProgramResult {
    Ok(())
}

use rush_svm::instruction::accounts::{Context, UpdateWorldAccounts};
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey};

/// Update World
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
pub fn process_update_world(
    program_id: &Pubkey,
    ctx: Context<UpdateWorldAccounts>,
) -> ProgramResult {
    Ok(())
}

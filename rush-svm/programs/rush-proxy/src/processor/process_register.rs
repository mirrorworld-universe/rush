use crate::instruction::accounts::{Context, RegisterAccounts};
use borsh::BorshSerialize;
use rush_svm::{pda::UserPDA, state::User};
use solana_program::{
    entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, rent::Rent,
    system_instruction, sysvar::Sysvar,
};

/// Register
///
/// - Creates a new account for User state
/// - Stores User state in newly created account
///
/// Accounts
/// 0. `[SIGNER]`       User Authority
/// 1. `[WRITE]`        User State PDA
/// 2. `[]`             World State PDA
/// 3. `[]`             System Program
///
/// Instruction Data
/// - user_agent_salt: String
/// - bump: u8
///
/// Data Validations
/// -
///
pub fn process_register(
    program_id: &Pubkey,
    ctx: Context<RegisterAccounts>,
    user_agent_salt: String,
    bump: u8,
) -> ProgramResult {
    let new_user_state = User::new(*ctx.accounts.user_authority.key, bump);
    let new_user_size = std::mem::size_of::<User>();
    let rent_exempt_cost = Rent::get()?.minimum_balance(new_user_size);
    let space_needed: u64 = new_user_size as u64;

    // build create_account instruction
    let create_user_account_ix = system_instruction::create_account(
        ctx.accounts.user_authority.key,
        ctx.accounts.user.key,
        rent_exempt_cost,
        space_needed,
        program_id,
    );

    // invoke CPI instruction
    invoke_signed(
        &create_user_account_ix,
        &[
            ctx.accounts.user_authority.clone(),
            ctx.accounts.user.clone(),
        ],
        &[&[
            UserPDA::TAG.as_bytes(),
            ctx.accounts.world.key.as_ref(),
            ctx.accounts.user_authority.key.as_ref(),
            user_agent_salt.as_bytes(),
            &[bump],
        ]],
    )?;

    // store new User state into newly created account
    let mut new_user_raw_bytes = ctx.accounts.user.try_borrow_mut_data()?;
    new_user_state.serialize(&mut &mut new_user_raw_bytes[..])?;

    Ok(())
}

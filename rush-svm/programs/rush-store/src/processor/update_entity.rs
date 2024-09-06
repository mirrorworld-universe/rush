use crate::instruction::accounts::{Context, UpdateEntityAccounts};
use borsh::BorshSerialize;
use rush_core::blueprint::{Component, ComponentValue};
use rush_svm::state::Instance;
use solana_program::{borsh1, entrypoint::ProgramResult, pubkey::Pubkey};

/// Update Entity
///
/// - Try data slice with Borsh1 Unchecked
/// - Update Instance State PDA with new component value
///
/// Accounts
/// 0. `[SIGNER]`       Instance Authority
/// 1. `[WRITE]`        Instance PDA
///
/// Data Validations
/// -
///
pub fn process_update_entity(
    _program_id: &Pubkey,
    ctx: Context<UpdateEntityAccounts>,
    component: Component,
    value: ComponentValue,
) -> ProgramResult {
    let mut instance_data = ctx.accounts.instance.try_borrow_mut_data()?;

    // need to use Borsh version 1 for dynamic data
    // else, de/serialization will fail with Account Unknown Error at runtime
    let mut instance = borsh1::try_from_slice_unchecked::<Instance>(&instance_data)?;

    // TODO: Error handling
    let component_mut = instance.components.get_mut(&component).unwrap();
    *component_mut = value;

    // store new Instance state
    instance.serialize(&mut &mut instance_data[..])?;

    Ok(())
}

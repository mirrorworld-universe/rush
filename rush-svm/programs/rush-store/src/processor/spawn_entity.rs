// use borsh::BorshSerialize;
// use rush_core::blueprint::{Component, ComponentValue, Entity, Region};
// use rush_svm::instruction::accounts::{Context, SpawnEntityAccounts};
// use rush_svm::{
//     pda::InstancePDA,
//     require,
//     state::{Instance, World},
// };
// use solana_program::{
//     borsh1, entrypoint::ProgramResult, hash::hash, program::invoke_signed,
//     program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
// };
// use std::collections::BTreeMap;
//
// /// Spawn Entity
// ///
// /// - Creates a new account for Instance state
// /// - Stores initial Instance state in newly created account
// ///
// /// Accounts
// /// 0. `[SIGNER]`       Instance Authority
// /// 1. `[WRITE]`        Instance
// /// 2. `[WRITE]`        World
// /// 3. `[]`             System Program
// ///
// /// Instruction Data
// /// - region: Region,
// /// - entity: Entity,
// /// - components: BTreeMap<Component, ComponentValue>,
// /// - nonce: u64,
// /// - bump: u8,
// ///
// /// Data Validations
// /// -
// ///
// pub fn process_spawn_entity(
//     program_id: &Pubkey,
//     ctx: Context<SpawnEntityAccounts>,
//     region: Region,
//     entity: Entity,
//     components: BTreeMap<Component, ComponentValue>,
//     nonce: u64,
//     bump: u8,
// ) -> ProgramResult {
//     // Make sure World exists
//     let world_data = ctx.accounts.world.try_borrow_data()?;
//     let mut world = borsh1::try_from_slice_unchecked::<World>(&world_data)?;
//     // World must be initialized
//     require!(
//         world.is_initialized(),
//         ProgramError::UninitializedAccount,
//         "world"
//     );
//
//     let new_instance_state = Instance::new(
//         components,
//         nonce,
//         *ctx.accounts.instance_authority.key,
//         bump,
//     );
//
//     // update world_data instances
//     //
//     // create_world guarantees existence of keys, unwrap here is okay
//     let instance_mut = world.instances.get_mut(&region).unwrap();
//     instance_mut.insert(key, value);
//
//     // need to use Borsh version 1 for dynamic data
//     // else, de/serialization will fail with Account Unknown Error at runtime
//     let new_instance_size = borsh1::get_instance_packed_len(&new_instance_state)?;
//
//     // rent from dynamic data size
//     let rent_exempt_cost = Rent::get()?.minimum_balance(new_instance_size);
//     // space from dynamic data size
//     let space_needed: u64 = new_instance_size as u64;
//
//     // build create_account instruction
//     let create_instance_account_ix = system_instruction::create_account(
//         ctx.accounts.instance_authority.key,
//         ctx.accounts.instance.key,
//         rent_exempt_cost,
//         space_needed,
//         program_id,
//     );
//
//     // invoke CPI instruction
//     invoke_signed(
//         &create_instance_account_ix,
//         &[
//             ctx.accounts.instance_authority.clone(),
//             ctx.accounts.instance.clone(),
//         ],
//         &[&[
//             InstancePDA::TAG.as_bytes(),
//             ctx.accounts.world.key.as_ref(),
//             region.as_bytes(),
//             entity.as_bytes(),
//             &nonce.to_le_bytes(),
//             &[bump],
//         ]],
//     )?;
//
//     // store new Instance state into newly created account
//     let mut new_instance_raw_bytes = ctx.accounts.instance.try_borrow_mut_data()?;
//     new_instance_state.serialize(&mut &mut new_instance_raw_bytes[..])?;
//
//     Ok(())
// }

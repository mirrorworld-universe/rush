use super::*;
use rush_svm::instruction::RushStoreInstruction;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    system_program::ID as SYSTEM_PROGRAM_ID,
    transaction::Transaction,
};

/// Create World
///
/// - Creates a new account for World state
/// - Stores initial World state in newly created account
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
/// 2. `[]`             System Program
///
/// Instruction Data
/// - name: String
/// - description: String
/// - regions: Vec<Region>,
/// - entities: Vec<Entity>,
/// - bump: u8
///
/// Data Validations
/// -
///

/// Test Happy Path
#[tokio::test]
async fn test_create_world() {
    // let program_id = Pubkey::new_unique();
    // let program_test = ProgramTest::new(
    //     // .so fixture is  retrieved from /target/deploy
    //     "rush_store",
    //     program_id,
    //     // shank is incompatible with instantiating the BuiltInFunction
    //     None,
    // );
    // let mut ctx = program_test.start_with_context().await;
    //
    // let (world_pda, world_bump) =
    //     Pubkey::find_program_address(&[b"state", ctx.payer.pubkey().as_ref()], &program_id);
    //
    // // create mint
    // let init_ix = RushStoreInstruction::CreateWorld {
    //     name: String::from("Sonic's World"),
    //     description: String::from("This is Sonic's World"),
    //     regions: vec![],
    //     entities: vec![],
    //     bump: (),
    // };
    // let mut init_ix_data = Vec::new();
    // init_ix.serialize(&mut init_ix_data).unwrap();
    //
    // let first_num = 1;
    // let second_num = 2;
    // let add_ix = ProgramInstruction::Add {
    //     a: first_num,
    //     b: second_num,
    // };
    // let mut add_ix_data = Vec::new();
    // add_ix.serialize(&mut add_ix_data).unwrap();
    //
    // let transaction = Transaction::new_signed_with_payer(
    //     &[
    //         Instruction {
    //             program_id,
    //             // Accounts
    //             //
    //             // 1. State
    //             // 2. Payer
    //             // 3. System Program (A program in Solana, is an account)
    //             accounts: vec![
    //                 AccountMeta::new(state_pda, false),
    //                 AccountMeta::new(ctx.payer.pubkey(), true),
    //                 AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    //             ],
    //             data: init_ix_data.clone(),
    //         },
    //         Instruction {
    //             program_id,
    //             // Accounts
    //             //
    //             // 1. Payer
    //             // 2. State
    //             accounts: vec![
    //                 AccountMeta::new(ctx.payer.pubkey(), true),
    //                 AccountMeta::new(state_pda, false),
    //             ],
    //             data: add_ix_data.clone(),
    //         },
    //     ],
    //     Some(&ctx.payer.pubkey()),
    //     &[&ctx.payer.insecure_clone()],
    //     ctx.last_blockhash,
    // );
    //
    // // send transaction
    // ctx.banks_client
    //     .process_transaction(transaction)
    //     .await
    //     .unwrap();
    //
    // // confirm state
    // let state = ctx
    //     .banks_client
    //     .get_account_data_with_borsh::<State>(state_pda)
    //     .await
    //     .unwrap();
    //
    // let sum: u64 = first_num + second_num;
    // assert_eq!(state.sum, sum);
}

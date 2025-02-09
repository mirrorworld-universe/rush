use rush_ecs_svm::{
    client::{ix_create_world, ix_update_world},
    pda::WorldPDA,
    state::World,
};
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction};

/// Update World
///
/// - Try data slice with Borsh1 Unchecked
/// - Update World State PDA with new entities / regions
///
/// Accounts
/// 0. `[SIGNER]`       World Authority
/// 1. `[WRITE]`        World PDA
///
/// Instruction Data
/// - regions: Vec<Region>,
/// - entities: Vec<Entity>
///
/// Data Validations
/// -
///

/// Test Happy Path
#[tokio::test]
async fn test_update_world() {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        // .so fixture is  retrieved from /target/deploy
        "rush_ecs_store",
        program_id,
        // shank is incompatible with instantiating the BuiltInFunction
        None,
    );
    let mut ctx = program_test.start_with_context().await;

    let name = String::from("Sonic's World");
    let description = String::from("This is Sonic's World");
    let regions = vec!["region1".to_string(), "region2".to_string()];
    let entities = vec!["entity1".to_string(), "entity2".to_string()];

    let (world_pda, world_bump) = WorldPDA::find_pda(&program_id, &name, &description);

    // CreateWorld
    let ix = ix_create_world(
        &program_id,
        name.clone(),
        description.clone(),
        regions.clone(),
        entities.clone(),
        world_bump,
        &world_pda,
        &ctx.payer.pubkey(),
        &ctx.payer.pubkey(),
    );

    let regions2 = vec![
        "region1".to_string(),
        "region2".to_string(),
        "region3".to_string(),
    ];
    let entities2 = vec![
        "entity1".to_string(),
        "entity2".to_string(),
        "entity3".to_string(),
    ];

    // UpdateWorld
    let ix2 = ix_update_world(
        &program_id,
        regions2.clone(),
        entities2.clone(),
        &world_pda,
        &ctx.payer.pubkey(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[ix, ix2],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer.insecure_clone()],
        ctx.last_blockhash,
    );

    // send transaction
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // confirm state
    let state = ctx
        .banks_client
        .get_account_data_with_borsh::<World>(world_pda)
        .await
        .unwrap();

    assert_eq!(state.name, name);
    assert_eq!(state.description, description);
    assert_eq!(state.regions, regions2);
    assert_eq!(state.entities, entities2);
    assert_eq!(state.world_authority, ctx.payer.pubkey());
    assert_eq!(state.bump, world_bump);
    assert!(!state.is_launched);
}

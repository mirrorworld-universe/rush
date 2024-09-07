use std::collections::BTreeMap;

use rush_core::blueprint::{Component, ComponentValue};
use rush_svm::{
    client::{ix_create_world, ix_spawn_entity},
    pda::{InstancePDA, WorldPDA},
    state::Instance,
};
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction};

/// Spawn Entity
///
/// - Creates a new account for Instance state
/// - Stores initial Instance state in newly created account
///
/// Accounts
/// 0. `[SIGNER]`       Instance Authority
/// 1. `[WRITE]`        Instance
/// 2. `[]`             World
/// 3. `[]`             System Program
///
/// Instruction Data
/// - region: Region,
/// - entity: Entity,
/// - components: BTreeMap<Component, ComponentValue>,
/// - nonce: u64,
/// - bump: u8,
///
/// Data Validations
/// -
///

/// Test Happy Path
#[tokio::test]
async fn test_spawn_entity() {
    let program_id = Pubkey::new_unique();
    let program_test = ProgramTest::new(
        // .so fixture is  retrieved from /target/deploy
        "rush_store",
        program_id,
        // shank is incompatible with instantiating the BuiltInFunction
        None,
    );
    let mut ctx = program_test.start_with_context().await;

    let name = String::from("Sonic's World");
    let description = String::from("This is Sonic's World");
    let regions = vec!["region1".to_string(), "region2".to_string()];
    let entities = vec!["entity1".to_string(), "entity2".to_string()];

    let (world_pda, world_bump) =
        WorldPDA::find_pda(&program_id, &name, &description, &ctx.payer.pubkey());
    let ix = ix_create_world(
        &program_id,
        name.clone(),
        description.clone(),
        regions.clone(),
        entities.clone(),
        world_bump,
        &world_pda,
        &ctx.payer.pubkey(),
    );

    let region = String::from("region1");
    let entity = String::from("entity");
    let mut components: BTreeMap<Component, ComponentValue> = BTreeMap::new();
    let value = 143;
    components.insert(String::from("x"), ComponentValue::Integer(value));
    components.insert(String::from("y"), ComponentValue::Integer(value));
    let nonce = 1;

    let (instance_pda, instance_bump) =
        InstancePDA::find_pda(&program_id, &world_pda, &region, &entity, nonce);

    let ix2 = ix_spawn_entity(
        &program_id,
        region,
        entity,
        components.clone(),
        nonce,
        instance_bump,
        &instance_pda,
        &ctx.payer.pubkey(),
        &world_pda,
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
        .get_account_data_with_borsh::<Instance>(instance_pda)
        .await
        .unwrap();

    state.is_initialized();
    assert_eq!(state.components, components);
    assert_eq!(state.nonce, nonce);
    assert_eq!(state.instance_authority, ctx.payer.pubkey());
    assert_eq!(state.bump, instance_bump);
}

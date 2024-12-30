use std::collections::BTreeMap;

use rush_ecs_core::blueprint::{Component, ComponentValue};
use rush_ecs_svm::{
    client::{ix_create_world, ix_spawn_entity},
    pda::{InstancePDA, WorldPDA},
    state::{Instance, World},
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

    let region = String::from("region1");
    let entity = String::from("entity1");
    let mut components: BTreeMap<Component, ComponentValue> = BTreeMap::new();
    let value = 143;
    components.insert(String::from("x"), ComponentValue::Integer(value));
    components.insert(String::from("y"), ComponentValue::Integer(value));
    let nonce = 1;

    let (instance_pda, instance_bump) =
        InstancePDA::find_pda(&program_id, &world_pda, &region, &entity, nonce);

    let ix2 = ix_spawn_entity(
        &program_id,
        region.clone(),
        entity.clone(),
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

    let world_state = ctx
        .banks_client
        .get_account_data_with_borsh::<World>(world_pda)
        .await
        .unwrap();

    assert_eq!(
        *world_state
            .instances
            .get(&region)
            .unwrap()
            .get(&entity)
            .unwrap(),
        nonce
    );

    // confirm state
    let instance_state = ctx
        .banks_client
        .get_account_data_with_borsh::<Instance>(instance_pda)
        .await
        .unwrap();

    instance_state.is_initialized();
    assert_eq!(instance_state.components, components);
    assert_eq!(instance_state.nonce, nonce);
    assert_eq!(instance_state.instance_authority, ctx.payer.pubkey());
    assert_eq!(instance_state.bump, instance_bump);
}

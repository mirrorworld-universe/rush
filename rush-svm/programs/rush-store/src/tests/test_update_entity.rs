use std::collections::BTreeMap;

use rush_core::blueprint::{Component, ComponentValue};
use rush_svm::{
    client::{ix_create_world, ix_spawn_entity, ix_update_entity},
    pda::{InstancePDA, WorldPDA},
    state::Instance,
};
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction};

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

/// Test Happy Path
#[tokio::test]
async fn test_update_entity() {
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
    let entity = String::from("entity");
    let mut components: BTreeMap<Component, ComponentValue> = BTreeMap::new();
    let value = ComponentValue::Integer(143);
    let new_value = ComponentValue::Integer(1337);
    let x_comp = String::from("x");
    let y_comp = String::from("y");
    components.insert(x_comp.clone(), value.clone());
    components.insert(y_comp.clone(), value.clone());
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

    let ix3 = ix_update_entity(
        &program_id,
        x_comp.clone(),
        new_value.clone(),
        &instance_pda,
        &ctx.payer.pubkey(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[ix, ix2, ix3],
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
    assert_eq!(
        state.components.get(&y_comp).unwrap(),
        components.get(&y_comp).unwrap()
    );
    assert_eq!(*state.components.get(&x_comp).unwrap(), new_value);
    assert_eq!(state.nonce, nonce);
    assert_eq!(state.instance_authority, ctx.payer.pubkey());
    assert_eq!(state.bump, instance_bump);
}

use super::client::{ix_proxy_create_world, ix_register};
use rush_svm::{
    pda::{UserPDA, WorldPDA},
    state::{User, World},
};
use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction};

#[tokio::test]
async fn test_proxy() {
    let program_id = Pubkey::new_unique();
    let store_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        // .so fixture is  retrieved from /target/deploy
        "rush_proxy",
        program_id,
        // shank is incompatible with instantiating the BuiltInFunction
        None,
    );
    program_test.add_program("rush_store", store_program_id, None);
    let mut ctx = program_test.start_with_context().await;

    let name = String::from("Sonic's World");
    let description = String::from("This is Sonic's World");
    let regions = vec!["region1".to_string(), "region2".to_string()];
    let entities = vec!["entity1".to_string(), "entity2".to_string()];
    let user_agent_salt = String::from("myuseragent");

    // IMPORTANT NOTE: Use the PROGRAM_ID of rush_store for WORLD_PDA, not rush_proxy
    let (world_pda, world_bump) =
        WorldPDA::find_pda(&store_program_id, &name, &description, &ctx.payer.pubkey());

    let (user_pda, user_bump) = UserPDA::find_pda(
        &program_id,
        &ctx.payer.pubkey(),
        &world_pda,
        user_agent_salt.clone(),
    );

    let ix_register = ix_register(
        &program_id,
        user_agent_salt.clone(),
        user_bump,
        &user_pda,
        &world_pda,
        &ctx.payer.pubkey(),
    );

    let ix_proxy_create_world = ix_proxy_create_world(
        &program_id,
        user_agent_salt,
        user_bump,
        name.clone(),
        description.clone(),
        regions.clone(),
        entities.clone(),
        world_bump,
        &ctx.payer.pubkey(),
        &user_pda,
        &world_pda,
        &store_program_id,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[ix_register, ix_proxy_create_world],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer.insecure_clone()],
        ctx.last_blockhash,
    );

    // send transaction
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // confirm User
    let user_state = ctx
        .banks_client
        .get_account_data_with_borsh::<User>(user_pda)
        .await
        .unwrap();

    assert!(user_state.is_initialized());
    assert_eq!(user_state.user_authority, ctx.payer.pubkey());
    assert_eq!(user_state.bump, user_bump);

    // confirm World
    let world_state = ctx
        .banks_client
        .get_account_data_with_borsh::<World>(world_pda)
        .await
        .unwrap();

    assert!(world_state.is_initialized());
    assert_eq!(world_state.name, name);
    assert_eq!(world_state.description, description);
    assert_eq!(world_state.regions, regions);
    assert_eq!(world_state.entities, entities);
    assert_eq!(world_state.world_authority, ctx.payer.pubkey());
    // println!(
    //     "DEBUG HERE:\nworld auth {} == user_pda {}",
    //     world_state.world_authority, user_pda
    // );
    // assert_eq!(world_state.world_authority, user_pda);
    assert_eq!(world_state.bump, world_bump);
    assert!(!world_state.is_launched);
}

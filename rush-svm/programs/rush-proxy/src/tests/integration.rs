use super::client::{ix_proxy_create_world, ix_register};
use rush_svm::{
    pda::{UserPDA, WorldPDA},
    state::{User, World},
};
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey, signature::Signer, signer::keypair::Keypair, system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

// [192,45,79,47,38,198,135,27,191,116,8,103,96,204,251,131,110,7,179,0,236,71,217,202,191,140,13,148,165,62,107,20,118,252,252,98,134,2,49,17,166,221,114,65,149,220,228,81,254,57,227,230,70,178,135,176,103,235,188,54,173,91,232,57]

#[tokio::test]
async fn test_proxy() {
    let signer = Keypair::from_bytes(&[
        192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
        236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49, 17,
        166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103, 235, 188,
        54, 173, 91, 232, 57,
    ])
    .unwrap();
    let program_id = Pubkey::from_str("1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM").unwrap();
    let store_program_id = Pubkey::from_str("1111111ogCyDbaRMvkdsHB3qfdyFYaG1WtRUAfdh").unwrap();
    let mut program_test = ProgramTest::new(
        // .so fixture is  retrieved from /target/deploy
        "rush_proxy",
        program_id,
        // shank is incompatible with instantiating the BuiltInFunction
        None,
    );
    program_test.add_program("rush_store", store_program_id, None);
    let mut ctx = program_test.start_with_context().await;

    let transfer_ix =
        system_instruction::transfer(&ctx.payer.pubkey(), &signer.pubkey(), 1000000000);
    let transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer.insecure_clone()],
        ctx.banks_client.get_latest_blockhash().await.unwrap(),
    );
    ctx.banks_client
        .process_transaction(transfer_tx)
        .await
        .unwrap();

    // TEST START

    let name = String::from("Sonic's World");
    let description = String::from("This is Sonic's World");
    let regions = vec!["region1".to_string(), "region2".to_string()];
    let entities = vec!["entity1".to_string(), "entity2".to_string()];
    let user_agent_salt = String::from("myuseragent");

    // IMPORTANT NOTE: Use the PROGRAM_ID of rush_store for WORLD_PDA, not rush_proxy
    let (world_pda, world_bump) =
        WorldPDA::find_pda(&store_program_id, &name, &description, &signer.pubkey());

    let (user_pda, user_bump) = UserPDA::find_pda(
        &program_id,
        &signer.pubkey(),
        &world_pda,
        user_agent_salt.clone(),
    );

    let ix_register = ix_register(
        &program_id,
        user_agent_salt.clone(),
        user_bump,
        &user_pda,
        &world_pda,
        &signer.pubkey(),
    );

    println!("Keypair: {:?}", signer);
    println!("Payer: {}", ctx.payer.pubkey());
    println!("Proxy Program Id: {program_id}");
    println!("Store Program Id: {store_program_id}");
    println!("World: {world_pda} {world_bump}");
    println!("User: {user_pda} {user_bump}");

    let ix_proxy_create_world = ix_proxy_create_world(
        &program_id,
        user_agent_salt,
        user_bump,
        name.clone(),
        description.clone(),
        regions.clone(),
        entities.clone(),
        world_bump,
        &signer.pubkey(),
        &user_pda,
        &world_pda,
        &store_program_id,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[ix_register, ix_proxy_create_world],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer.insecure_clone(), &signer],
        ctx.banks_client.get_latest_blockhash().await.unwrap(),
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
    assert_eq!(user_state.user_authority, signer.pubkey());
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
    assert_eq!(world_state.world_authority, signer.pubkey());
    // println!(
    //     "DEBUG HERE:\nworld auth {} == user_pda {}",
    //     world_state.world_authority, user_pda
    // );
    // assert_eq!(world_state.world_authority, user_pda);
    assert_eq!(world_state.bump, world_bump);
    assert!(!world_state.is_launched);
}

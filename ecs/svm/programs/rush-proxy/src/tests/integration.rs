use super::client;
use crate::instruction::RushProxyInstruction;
use borsh::{BorshDeserialize, BorshSerialize};
use rush_ecs_svm::{
    pda::{UserPDA, WorldPDA},
    state::{User, World},
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::{keypair_from_seed, Keypair},
    system_instruction,
    system_program::ID as SYSTEM_PROGRAM_ID,
    transaction::Transaction,
};
use std::str::FromStr;

// [192,45,79,47,38,198,135,27,191,116,8,103,96,204,251,131,110,7,179,0,236,71,217,202,191,140,13,148,165,62,107,20,118,252,252,98,134,2,49,17,166,221,114,65,149,220,228,81,254,57,227,230,70,178,135,176,103,235,188,54,173,91,232,57]

#[tokio::test]
async fn test_proxy() {
    let user_authority = keypair_from_seed(&[
        192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
        236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49, 17,
        166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103, 235, 188,
        54, 173, 91, 232, 57,
    ])
    .unwrap();

    // let proxy_program_id = Pubkey::new_unique();
    // let store_program_id = Pubkey::new_unique();
    let proxy_program_id = Pubkey::from_str("1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM").unwrap();
    let store_program_id = Pubkey::from_str("1111111ogCyDbaRMvkdsHB3qfdyFYaG1WtRUAfdh").unwrap();
    let mut test = ProgramTest::default();
    test.add_program("rush_ecs_proxy", proxy_program_id, None);
    test.add_program("rush_ecs_store", store_program_id, None);

    // TEST START

    let name = String::from("Sonic's World");
    let description = String::from("This is Sonic's World");
    let regions = vec!["region1".to_string(), "region2".to_string()];
    let entities = vec!["entity1".to_string(), "entity2".to_string()];
    let user_agent_salt = String::from("myuseragent");

    // IMPORTANT NOTE: Use the PROGRAM_ID of rush_ecs_store for WORLD_PDA, not rush_proxy
    let (world_pda, world_bump) = WorldPDA::find_pda(&store_program_id, &name, &description);

    println!("PROXY PROGRAM ID: {}", proxy_program_id);

    let (user_pda, user_bump) = UserPDA::find_pda(
        &proxy_program_id,
        &user_authority.pubkey(),
        &world_pda,
        user_agent_salt.clone(),
    );
    println!("FOUND USER PDA: {}", user_pda);

    let created_user_pda = UserPDA::create_pda(
        &proxy_program_id,
        &user_authority.pubkey(),
        &world_pda,
        user_agent_salt.clone(),
        user_bump,
    );
    println!("CREATED USER PDA: {}", created_user_pda);

    let user = User::new(user_authority.pubkey(), user_bump);
    let mut user_account_data: Vec<u8> = Vec::new();
    user.serialize(&mut user_account_data).unwrap();
    let mut user_state_account =
        Account::new(u32::MAX as u64, user_account_data.len(), &proxy_program_id);
    user_state_account.data = user_account_data;

    test.add_account(user_pda, user_state_account);

    let mut ctx = test.start_with_context().await;

    // let ix_proxy_create_world = client::ix_proxy_create_world(
    //     &proxy_program_id,
    //     user_agent_salt,
    //     user_bump,
    //     name.clone(),
    //     description.clone(),
    //     regions.clone(),
    //     entities.clone(),
    //     world_bump,
    //     &user_authority.pubkey(),
    //     &user_pda,
    //     &world_pda,
    //     &store_program_id,
    // );

    let instruction = RushProxyInstruction::ProxyCreateWorld {
        user_agent_salt,
        user_bump,
        name,
        description,
        regions,
        entities,
        world_bump,
    };

    let ix_proxy_create_world = Instruction::new_with_borsh(
        proxy_program_id,
        &instruction,
        vec![
            AccountMeta::new_readonly(user_pda, false),
            AccountMeta::new(user_authority.pubkey(), true),
            AccountMeta::new(world_pda, false),
            AccountMeta::new_readonly(store_program_id, false),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
    );

    let transaction = Transaction::new_signed_with_payer(
        &[ix_proxy_create_world],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer.insecure_clone(), &user_authority],
        ctx.banks_client.get_latest_blockhash().await.unwrap(),
    );

    // send transaction
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // confirm World
    // let world_state = ctx
    //     .banks_client
    //     .get_account_data_with_borsh::<World>(world_pda)
    //     .await
    //     .unwrap();
    //
    // assert!(world_state.is_initialized());
    // assert_eq!(world_state.name, name);
    // assert_eq!(world_state.description, description);
    // assert_eq!(world_state.regions, regions);
    // assert_eq!(world_state.entities, entities);
    // assert_eq!(world_state.world_authority, user_pda);
    // assert_eq!(world_state.bump, world_bump);
    // assert!(!world_state.is_launched);
}

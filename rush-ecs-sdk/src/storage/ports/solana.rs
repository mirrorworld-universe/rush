use crate::{error::StorageError, storage::Storage};
use anyhow::{bail, Result};
use borsh::BorshDeserialize;
use colored::Colorize;
use rush_ecs_core::blueprint::{Blueprint, Component, ComponentValue, Entity, Region};
use rush_ecs_parser::{toml::TomlParser, Loader};
use rush_ecs_svm::{
    client::{ix_create_world, ix_spawn_entity, ix_update_entity},
    pda::{InstancePDA, WorldPDA},
    state::{Instance, World},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},
    transaction::Transaction,
};
use std::path::Path;

// #[derive(Clone, Debug, Default, Eq, PartialEq)]
#[derive(Debug, PartialEq)]
pub struct Solana {
    pub blueprint: Blueprint,
    pub program_id: Pubkey,
    pub signer: Keypair,
    pub rpc_url: String,
}

// TODO: Fix data type
impl Solana {
    pub fn new(program_id: Pubkey, signer: Keypair, rpc_url: String, path: &str) -> Self {
        // TODO: Support other parsers. Pinned to TOML for now
        let toml_parser = TomlParser {};
        let loader = Loader::new(toml_parser);
        let path = Path::new(path);
        let blueprint = loader
            .load_blueprint(path)
            .expect("Expected a valid blueprint path");

        Self {
            blueprint,
            program_id,
            signer,
            rpc_url,
        }
    }
}

impl Storage for Solana {
    fn migrate(&mut self) -> Result<()> {
        let client = RpcClient::new(self.rpc_url.clone());

        let regions = self.blueprint.regions.keys().cloned().collect::<Vec<_>>();
        let entities = self.blueprint.entities.keys().cloned().collect::<Vec<_>>();
        let (world_pda, world_bump) = WorldPDA::find_pda(
            &self.program_id,
            self.blueprint.name.as_str(),
            self.blueprint.description.as_str(),
        );

        let ix = ix_create_world(
            &self.program_id,
            self.blueprint.name.clone(),
            self.blueprint.description.clone(),
            regions.clone(),
            entities.clone(),
            world_bump,
            &world_pda,
            &self.signer.pubkey(),
            &self.signer.pubkey(),
        );

        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction(&tx)?;

        println!(
            "[{}] Created world: {}, Signature: {}",
            "SUCCESS".green().bold(),
            world_pda,
            signature
        );

        // push spawn_entity instructions
        for region_name in regions.iter() {
            for entity_name in entities.iter() {
                // Blueprint preload ensures that unwrap is ok here
                let instances = self
                    .blueprint
                    .instances
                    .get(region_name)
                    .unwrap()
                    .get(entity_name)
                    .unwrap();

                for (each_index, each_instance) in instances.iter().enumerate() {
                    let nonce = each_index as u64 + 1;
                    let (instance_pda, instance_bump) = InstancePDA::find_pda(
                        &self.program_id,
                        &world_pda,
                        region_name,
                        entity_name,
                        nonce,
                    );

                    let ix = ix_spawn_entity(
                        &self.program_id,
                        region_name.to_string(),
                        entity_name.to_string(),
                        each_instance.clone(),
                        nonce,
                        instance_bump,
                        &instance_pda,
                        &self.signer.pubkey(),
                        &world_pda,
                    );

                    let recent_blockhash = client.get_latest_blockhash()?;
                    let tx = Transaction::new_signed_with_payer(
                        &[ix],
                        Some(&self.signer.pubkey()),
                        &[&self.signer],
                        recent_blockhash,
                    );
                    let signature = client.send_and_confirm_transaction(&tx)?;
                    println!(
                        "[{}] Spawned #{}: {}, Signature: {}",
                        "SUCCESS".green().bold(),
                        nonce,
                        instance_pda,
                        signature
                    );
                }
            }
        }

        Ok(())
    }

    fn create(&mut self, region: Region, entity: Entity) -> Result<u64> {
        let client = RpcClient::new(self.rpc_url.clone());

        // fetch nonce
        let (world_pda, _) = WorldPDA::find_pda(
            &self.program_id,
            self.blueprint.name.as_str(),
            self.blueprint.description.as_str(),
        );
        let world_account_data = client.get_account_data(&world_pda)?;
        let world = World::try_from_slice(&world_account_data)?;
        // TODO: Consider using the nonce internally in spawn_entity instruction
        let nonce = world.instances.get(&region).unwrap().get(&entity).unwrap() + 1;

        let default_components = self.blueprint.get_default_components(&entity).unwrap();
        let (instance_pda, instance_bump) =
            InstancePDA::find_pda(&self.program_id, &world_pda, &region, &entity, nonce);

        let ix = ix_spawn_entity(
            &self.program_id,
            region,
            entity,
            default_components,
            nonce,
            instance_bump,
            &instance_pda,
            &self.signer.pubkey(),
            &world_pda,
        );

        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            recent_blockhash,
        );
        let signature = client.send_and_confirm_transaction(&tx)?;

        println!(
            "[{}] Spawned #{}: {}, Signature: {}",
            "SUCCESS".green().bold(),
            nonce,
            instance_pda,
            signature
        );

        Ok(nonce)
    }

    // TODO: Implement Delete instance
    fn delete(&mut self, region: Region, entity: Entity, nonce: u64) -> Result<()> {
        Ok(())
    }

    fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        let client = RpcClient::new(self.rpc_url.clone());

        let (world_pda, _) = WorldPDA::find_pda(
            &self.program_id,
            self.blueprint.name.as_str(),
            self.blueprint.description.as_str(),
        );

        let (instance_pda, _) =
            InstancePDA::find_pda(&self.program_id, &world_pda, &region, &entity, nonce);

        let data = client.get_account_data(&instance_pda)?;
        let instance_state = Instance::try_from_slice(&data)?;
        let value = instance_state.components.get(&component).unwrap().clone();

        println!(
            "[{}] Fetching #{}: {}",
            "SUCCESS".green().bold(),
            nonce,
            instance_pda
        );

        Ok(value)
    }

    fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        let client = RpcClient::new(self.rpc_url.clone());

        let (world_pda, _) = WorldPDA::find_pda(
            &self.program_id,
            self.blueprint.name.as_str(),
            self.blueprint.description.as_str(),
        );

        let (instance_pda, _) =
            InstancePDA::find_pda(&self.program_id, &world_pda, &region, &entity, nonce);

        let ix = ix_update_entity(
            &self.program_id,
            component,
            value,
            &instance_pda,
            &self.signer.pubkey(),
        );

        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.signer.pubkey()),
            &[&self.signer],
            recent_blockhash,
        );
        let signature = client.send_and_confirm_transaction(&tx)?;

        println!(
            "[{}] Updating #{}: {}, Signature: {}",
            "SUCCESS".green().bold(),
            nonce,
            instance_pda,
            signature
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use borsh::BorshDeserialize;
    use rush_ecs_svm::state::Instance;
    use solana_program_test::*;
    use solana_sdk::{
        borsh1,
        signer::{keypair::Keypair, SeedDerivable},
    };
    use std::str::FromStr;

    // Happy path
    #[test]
    fn test_solana_migrate() {
        // prepare test context
        let program_id = Pubkey::from_str("FXm4HiySCyKv3HrynYKY7yfanyH7dJGMuvxXsbnvtW5c").unwrap();
        let seed = [
            192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
            236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49,
            17, 166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103,
            235, 188, 54, 173, 91, 232, 57,
        ];

        let signer = Keypair::from_seed(&seed).unwrap();

        let path_str = "fixtures/blueprint.toml";
        let loader = Loader::new(TomlParser {});
        let path = Path::new(path_str);
        let blueprint = loader.load_blueprint(path).unwrap();

        let (world_pda, world_bump) =
            WorldPDA::find_pda(&program_id, &blueprint.name, &blueprint.description);

        let rpc_url = String::from("http://127.0.0.1:8899");
        let client = RpcClient::new(rpc_url.clone());
        let mut solana = Solana::new(program_id, signer.insecure_clone(), rpc_url, path_str);

        solana.migrate().unwrap();

        let data = client.get_account_data(&world_pda).unwrap();
        let state = borsh1::try_from_slice_unchecked::<World>(&data).unwrap();

        assert!(state.is_initialized());
        assert_eq!(state.name, blueprint.name);
        assert_eq!(state.description, blueprint.description);
        assert_eq!(
            state.regions,
            blueprint.regions.into_keys().collect::<Vec<_>>()
        );
        assert_eq!(
            state.entities,
            blueprint.entities.into_keys().collect::<Vec<_>>()
        );
        assert_eq!(state.world_authority, signer.pubkey());
        assert_eq!(state.bump, world_bump);
        assert!(!state.is_launched);
    }

    // Happy path
    #[test]
    fn test_solana_create() {
        // prepare test context
        let program_id = Pubkey::from_str("FXm4HiySCyKv3HrynYKY7yfanyH7dJGMuvxXsbnvtW5c").unwrap();
        let seed = [
            192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
            236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49,
            17, 166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103,
            235, 188, 54, 173, 91, 232, 57,
        ];

        let signer = Keypair::from_seed(&seed).unwrap();

        let path_str = "fixtures/blueprint.toml";
        let loader = Loader::new(TomlParser {});
        let path = Path::new(path_str);
        let blueprint = loader.load_blueprint(path).unwrap();

        let (world_pda, world_bump) =
            WorldPDA::find_pda(&program_id, &blueprint.name, &blueprint.description);

        let rpc_url = String::from("http://127.0.0.1:8899");
        let client = RpcClient::new(rpc_url.clone());
        let mut solana = Solana::new(program_id, signer.insecure_clone(), rpc_url, path_str);

        solana.migrate().unwrap();
        let region = "farm".to_string();
        let entity = "player".to_string();

        solana.create(region.clone(), entity.clone()).unwrap();

        let expected_nonce = 2;

        let data = client.get_account_data(&world_pda).unwrap();
        let world_state = borsh1::try_from_slice_unchecked::<World>(&data).unwrap();

        assert_eq!(
            *world_state
                .instances
                .get(&region)
                .unwrap()
                .get(&entity)
                .unwrap(),
            expected_nonce
        );

        let (instance_pda, instance_bump) =
            InstancePDA::find_pda(&program_id, &world_pda, &region, &entity, expected_nonce);
        let data = client.get_account_data(&instance_pda).unwrap();
        let instance_state = borsh1::try_from_slice_unchecked::<Instance>(&data).unwrap();
        let default_components = blueprint.get_default_components(&entity).unwrap();
        assert_eq!(instance_state.components, default_components);
        assert_eq!(instance_state.nonce, expected_nonce);
        assert_eq!(instance_state.instance_authority, signer.pubkey());
        assert_eq!(instance_state.bump, instance_bump);
    }

    // Happy path
    #[test]
    fn test_solana_get() {
        // prepare test context
        let program_id = Pubkey::from_str("FXm4HiySCyKv3HrynYKY7yfanyH7dJGMuvxXsbnvtW5c").unwrap();
        let seed = [
            192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
            236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49,
            17, 166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103,
            235, 188, 54, 173, 91, 232, 57,
        ];

        let signer = Keypair::from_seed(&seed).unwrap();

        let path_str = "fixtures/blueprint.toml";
        let loader = Loader::new(TomlParser {});
        let path = Path::new(path_str);
        let blueprint = loader.load_blueprint(path).unwrap();

        let (world_pda, world_bump) =
            WorldPDA::find_pda(&program_id, &blueprint.name, &blueprint.description);

        let rpc_url = String::from("http://127.0.0.1:8899");
        let mut solana = Solana::new(program_id, signer.insecure_clone(), rpc_url, path_str);

        solana.migrate().unwrap();

        let region = "farm".to_string();
        let entity = "apple".to_string();
        let component = "x".to_string();

        let component_value = solana.get(region, entity, 1, component).unwrap();

        let expected_parameter = 0;
        assert_matches!(component_value, ComponentValue::Integer(expected_parameter));
    }

    // Happy path
    #[test]
    fn test_solana_set() {
        // prepare test context
        let program_id = Pubkey::from_str("FXm4HiySCyKv3HrynYKY7yfanyH7dJGMuvxXsbnvtW5c").unwrap();
        let seed = [
            192, 45, 79, 47, 38, 198, 135, 27, 191, 116, 8, 103, 96, 204, 251, 131, 110, 7, 179, 0,
            236, 71, 217, 202, 191, 140, 13, 148, 165, 62, 107, 20, 118, 252, 252, 98, 134, 2, 49,
            17, 166, 221, 114, 65, 149, 220, 228, 81, 254, 57, 227, 230, 70, 178, 135, 176, 103,
            235, 188, 54, 173, 91, 232, 57,
        ];

        let signer = Keypair::from_seed(&seed).unwrap();

        let path_str = "fixtures/blueprint.toml";
        let loader = Loader::new(TomlParser {});
        let path = Path::new(path_str);
        let blueprint = loader.load_blueprint(path).unwrap();

        let (world_pda, world_bump) =
            WorldPDA::find_pda(&program_id, &blueprint.name, &blueprint.description);

        let rpc_url = String::from("http://127.0.0.1:8899");
        let client = RpcClient::new(rpc_url.clone());
        let mut solana = Solana::new(program_id, signer.insecure_clone(), rpc_url, path_str);

        solana.migrate().unwrap();

        let region = "farm".to_string();
        let entity = "player".to_string();
        let component = "x".to_string();
        let nonce = 1;
        let value = ComponentValue::Integer(143);

        solana
            .set(
                region.clone(),
                entity.clone(),
                nonce,
                component.clone(),
                value,
            )
            .unwrap();

        let (instance_pda, _) =
            InstancePDA::find_pda(&program_id, &world_pda, &region, &entity, nonce);

        let data = client.get_account_data(&instance_pda).unwrap();
        let instance_state = Instance::try_from_slice(&data).unwrap();
        let component_value = instance_state.components.get(&component).unwrap().clone();
        assert_matches!(component_value, value);
    }
}

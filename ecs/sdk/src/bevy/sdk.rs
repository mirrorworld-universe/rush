use std::str::FromStr;

use crate::auth::{Auth, FilesystemAuth};
use crate::storage::{Solana, Storage};
use anyhow::Result;
use rush_ecs_core::blueprint::{Component, ComponentValue, Entity, Region};
use solana_sdk::{pubkey::Pubkey, signer::keypair::Keypair};

pub struct BevySDK {
    keypair: Keypair,
    storage: Box<dyn Storage>,
}

impl BevySDK {
    pub fn new(
        rpc_url: String,
        program_id: &str,
        blueprint_path: &str,
        keypair_path: &str,
    ) -> Self {
        let auth = FilesystemAuth::new();
        let keypair = auth
            .signin(keypair_path)
            .expect("Expected a valid Keypair Path");
        let program_id_pubkey = Pubkey::from_str(program_id).expect("Expected a valid Program ID");

        let storage = Solana::new(
            program_id_pubkey,
            keypair.insecure_clone(),
            rpc_url,
            blueprint_path,
        );

        Self {
            keypair,
            storage: Box::new(storage),
        }
    }

    pub fn migrate(&mut self) -> Result<()> {
        self.storage.migrate()
    }

    pub fn create(&mut self, region: Region, entity: Entity) -> Result<u64> {
        self.storage.create(region, entity)
    }

    pub fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        self.storage.get(region, entity, nonce, component)
    }

    pub fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        self.storage.set(region, entity, nonce, component, value)
    }

    pub fn signin(&self) -> Keypair {
        // TODO: Temporary
        self.keypair.insecure_clone()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_sdk_integration() {
        let sdk = BevySDK::new(
            "https://devnet.sonic.game".to_string(),
            "8npxEZiWoi6zcBQ4Pw2e5enC1Av4UhzA2ZtPn1fKeciU",
            "fixtures/blueprint.toml",
            "/Users/kquirapas/.config/solana/id.json",
        );
    }
}

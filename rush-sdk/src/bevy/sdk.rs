use crate::auth::Auth;
use crate::storage::Storage;
use anyhow::Result;
use rush_core::blueprint::{Component, ComponentValue, Entity, Region};
use solana_sdk::signer::keypair::Keypair;

pub struct BevySDK {
    blueprint_path: String,
    keypair_path: String,
    auth: Box<dyn Auth>,
    storage: Box<dyn Storage>,
}

impl BevySDK {
    pub fn new(
        blueprint_path: String,
        keypair_path: String,
        auth: impl Auth,
        storage: impl Storage,
    ) -> Self {
        Self {
            blueprint_path,
            keypair_path,
            auth: Box::new(auth),
            storage: Box::new(storage),
        }
    }

    pub async fn migrate(&mut self) -> Result<()> {
        self.storage.migrate().await
    }

    pub async fn create(&mut self, region: Region, entity: Entity) -> Result<u64> {
        self.storage.create(region, entity).await
    }

    pub async fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        self.storage.get(region, entity, nonce, component).await
    }

    pub async fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        self.storage
            .set(region, entity, nonce, component, value)
            .await
    }

    pub fn signin(&self, path: &str) -> Result<Keypair> {
        self.auth.signin(path)
    }
}

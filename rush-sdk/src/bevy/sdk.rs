use crate::auth::Auth;
use crate::storage::Storage;
use anyhow::Result;
use async_trait::async_trait;

pub struct BevySDK {
    world_path: String,
    keypair_path: String,
    auth: Box<dyn Auth>,
    storage: Box<dyn Storage>,
}

impl BevySDK {
    pub fn new(
        world_path: String,
        keypair_path: String,
        auth: impl Auth,
        storage: impl Storage,
    ) -> Self {
        Self {
            world_path,
            keypair_path,
            auth: Box::new(auth),
            storage: Box::new(storage),
        }
    }
}

// impl Auth for BevySDK {
//     fn signin(&self, path: &str) -> Result<Keypair> {
//     }
// }

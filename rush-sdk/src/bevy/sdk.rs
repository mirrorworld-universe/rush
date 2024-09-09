use crate::auth::IAuth;
use crate::storage::IStorage;

pub struct BevySDK {
    world_path: String,
    keypair_path: String,
    auth: Box<dyn IAuth>,
    storage: Box<dyn IStorage>,
}

impl BevySDK {
    pub fn new(
        world_path: String,
        keypair_path: String,
        auth: impl IAuth,
        storage: impl IStorage,
    ) -> Self {
        Self {
            world_path,
            keypair_path,
            auth: Box::new(auth),
            storage: Box::new(storage),
        }
    }
}

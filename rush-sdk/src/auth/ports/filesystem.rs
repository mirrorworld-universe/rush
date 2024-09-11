use crate::{auth::Auth, error::AuthError};
use anyhow::{bail, Result};
use solana_sdk::signer::keypair::{read_keypair_file, Keypair};
use std::path::Path;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FilesystemAuth {}

impl FilesystemAuth {
    pub fn new() -> Self {
        Self {}
    }
}

impl Auth for FilesystemAuth {
    fn signin(&self, path: &str) -> Result<Keypair> {
        let key_path = Path::new(path);
        let keypair = read_keypair_file(key_path);

        match keypair {
            Ok(k) => Ok(k),
            Err(err) => bail!(AuthError::KeypairNotFound(err.to_string())),
        }
    }
}

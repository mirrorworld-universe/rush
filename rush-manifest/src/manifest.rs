//! Rush Manifest definition and utils
//!
//! Filename: Rush.toml

use crate::error::{utils::ensure_syntax, ManifestError};
use anyhow::{bail, Result};
use std::{collections::BTreeMap, fs::read_to_string, path::Path};
use toml::Table;

pub enum Repository {
    InMemory,
    Solana,
}

pub enum Chain {
    Solana {
        proxy: String,
        store: String,
        rpc: String,
        websocket: String,
        keypair: String,
    },
}

pub struct Manifest {
    name: String,
    storage: Repository,
    chain: Chain,
}

impl Manifest {
    const FILENAME: &'static str = "Rush.toml";

    pub fn new_solana(name: String, storage: Repository) -> Self {
        Self {
            name,
            storage,
            chain: Chain::Solana {
                proxy: String::default(),
                store: String::default(),
                rpc: String::default(),
                websocket: String::default(),
                keypair: String::default(),
            },
        }
    }

    // TODO: (REVIEW) Consider creating a trait for parsing
    pub fn from_toml(path: &str) -> Result<Manifest> {
        let manifest_path = match Path::new(path).canonicalize() {
            Ok(p) => p,
            Err(e) => bail!(e),
        };

        let manifest_string = read_to_string(manifest_path)?;

        println!("Manifest String: {}", manifest_string);

        let table: Table = match manifest_string.parse::<Table>() {
            Ok(t) => t,
            Err(e) => bail!(e),
        };

        let workspace_table = match table["workspace"].as_table() {
            Some(t) => t,
            None => bail!(ManifestError::MissingTable("workspace".to_string())),
        };

        // Validate Rush manifest syntax
        ensure_syntax(
            "Workspace must have a name".to_string(),
            workspace_table.contains_key("name"),
        );

        Ok(Self::new_solana("dummy".to_string(), Repository::Solana))
    }

    // pub save_toml(path: String) -> Result<Manifest> {
    //     let manifest_path = match Path::new(path).canonicalize() {
    //         Ok(p) => p,
    //         Err(e) => bail!(e),
    //     };
    //
    //     let manifest_string = read_to_string(manifest_path)?;
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_toml() {
        assert!(true);
    }

    #[test]
    fn test_save_toml() {
        assert!(true);
    }
}

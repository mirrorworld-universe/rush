//! Rush Manifest definition and utils
//!
//! Filename: Rush.toml

use crate::error::{utils::ensure_syntax, ManifestError};
use anyhow::{bail, Result};
use std::{fs::read_to_string, path::Path};
use toml::Table;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Repository {
    InMemory,
    Solana,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Chain {
    Solana {
        // proxy: String,
        store: String,
        rpc: String,
        // websocket: String,
        keypair: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
                // proxy: String::default(),
                store: String::default(),
                rpc: String::default(),
                // websocket: String::default(),
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

        /*
         * Workspace Table
         */

        let workspace_table = match table["workspace"].as_table() {
            Some(t) => t,
            None => bail!(ManifestError::MissingTable("workspace".to_string())),
        };

        ensure_syntax(
            "Workspace must have a name".to_string(),
            workspace_table.contains_key("name"),
        );

        // unwrap, ok
        let name = workspace_table.get("name").unwrap().as_str().unwrap();

        /*
         * Storage Table
         */

        let storage_table = match table["storage"].as_table() {
            Some(t) => t,
            None => bail!(ManifestError::MissingTable("storage".to_string())),
        };

        ensure_syntax(
            "Workspace storage must have a repository".to_string(),
            storage_table.contains_key("repository"),
        );

        // unwrap, ok
        let repo_value = storage_table.get("repository").unwrap().as_str().unwrap();
        let repository = Self::parse_repository(repo_value.to_string())?;

        let mut manifest = Self::new_solana(name.to_string(), repository);

        /*
         * Solana Table
         */

        let solana_table = match table["solana"].as_table() {
            Some(t) => t,
            None => bail!(ManifestError::MissingTable("solana".to_string())),
        };

        // ensure_syntax(
        //     "Solana table must have a proxy".to_string(),
        //     solana_table.contains_key("proxy"),
        // );
        ensure_syntax(
            "Solana table must have a store".to_string(),
            solana_table.contains_key("store"),
        );
        ensure_syntax(
            "Solana table must have an rpc".to_string(),
            solana_table.contains_key("rpc"),
        );
        // ensure_syntax(
        //     "Solana table must have a websocket".to_string(),
        //     solana_table.contains_key("websocket"),
        // );
        ensure_syntax(
            "Solana table must have a keypair".to_string(),
            solana_table.contains_key("keypair"),
        );

        // TODO: Remove unwraps
        let store = solana_table
            .get("store")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let rpc = solana_table
            .get("rpc")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        // let websocket_url = solana_table.get("websocket").unwrap().as_str().unwrap().to_string();
        let keypair = solana_table
            .get("keypair")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        manifest.chain = Chain::Solana {
            store,
            rpc,
            keypair,
        };

        Ok(manifest)
    }

    // pub save_toml(path: String) -> Result<Manifest> {
    //     let manifest_path = match Path::new(path).canonicalize() {
    //         Ok(p) => p,
    //         Err(e) => bail!(e),
    //     };
    //
    //     let manifest_string = read_to_string(manifest_path)?;
    // }

    pub fn parse_repository(repo_string: String) -> Result<Repository> {
        let repo = match repo_string.as_str() {
            "solana" => Repository::Solana,
            "memory" => Repository::InMemory,
            _ => bail!(ManifestError::UnsupportedRepo(repo_string)),
        };

        Ok(repo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_toml() {
        let manifest = Manifest::from_toml("fixtures/Rush.toml").unwrap();
        assert_eq!(manifest.name, "WORKSPACE");
        assert_eq!(manifest.storage, Repository::Solana);
        assert_eq!(
            manifest.chain,
            Chain::Solana {
                store: "STORE".to_string(),
                rpc: "RPC".to_string(),
                keypair: "KEYPAIR".to_string()
            }
        );
    }

    #[test]
    fn test_save_toml() {
        assert!(true);
    }
}

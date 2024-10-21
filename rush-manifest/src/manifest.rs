//! Rush Manifest definition and utils
//!
//! Filename: Rush.toml

use crate::error::{utils::ensure_syntax, ManifestError};
use anyhow::{bail, Result};
use std::{
    convert::From,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};
use toml::Table;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Repository {
    InMemory,
    Solana,
}

impl From<Repository> for String {
    fn from(repository: Repository) -> Self {
        match repository {
            Repository::InMemory => "memory".to_string(),
            Repository::Solana => "solana".to_string(),
        }
    }
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
    pub name: String,
    pub storage: Repository,
    pub chain: Chain,
}

impl Manifest {
    pub const FILENAME: &'static str = "Rush.toml";

    pub fn new_solana(name: String) -> Self {
        Self {
            name,
            storage: Repository::Solana,
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

        let mut manifest = Self::new_solana(name.to_string());
        manifest.storage = repository;

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

    pub fn save_toml(manifest: Manifest, path: &str) -> Result<()> {
        let name = manifest.name;
        let repo: String = manifest.storage.into();
        let Chain::Solana {
            store,
            rpc,
            keypair,
        } = manifest.chain;

        let filename = Self::FILENAME;

        let path = Path::new(path);
        let manifest_path = path.join(filename);
        let mut toml_file = File::create(manifest_path)?;

        // TODO: (REVIEW) Find better formatting (r#?)
        toml_file.write_all(
            format!("[workspace]\nname = \"{name}\"\n\n[storage]\nrepository = \"{repo}\"\n\n[solana]\nstore = \"{store}\"\nrpc = \"{rpc}\"\nkeypair = \"{keypair}\"",
            ).as_bytes())?;

        Ok(())
    }

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
    // TODO: Add string matching for test
    fn test_save_toml_without_trailing_slash() {
        let mut manifest = Manifest::new_solana("WORKSPACE".to_string());
        let store = "STORE".to_string();
        let rpc = "RPC".to_string();
        let keypair = "KEYPAIR".to_string();

        manifest.chain = Chain::Solana {
            store: store.clone(),
            rpc: rpc.clone(),
            keypair: keypair.clone(),
        };

        Manifest::save_toml(manifest, "fixtures/save").unwrap();
    }

    #[test]
    // TODO: Add string matching for test
    fn test_save_toml_with_trailing_slash() {
        let mut manifest = Manifest::new_solana("WORKSPACE".to_string());
        let store = "STORE".to_string();
        let rpc = "RPC".to_string();
        let keypair = "KEYPAIR".to_string();

        manifest.chain = Chain::Solana {
            store: store.clone(),
            rpc: rpc.clone(),
            keypair: keypair.clone(),
        };

        Manifest::save_toml(manifest, "fixtures/save/").unwrap();
    }
}

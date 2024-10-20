//! Rush Manifest definition and utils
//!
//! Filename: Rush.toml

use anyhow::{Result,bail};
use std::{collections::BTreeMap, fs::read_to_string, path::Path};
use toml::Table;
use crate::{utils::ensure_syntax, error::*};

pub struct Manifest {
    name: String,
    storage: Repository,
    chain: Chain,
}

impl Manifest {
    const FILENAME: &'static str = "Rush.toml";

    pub new_solana(name: String, storage: Repository) -> Self {
        Self {
            name,
            storage,
            chain: Solana
        }
    }

    // TODO: (REVIEW) Consider creating a trait for parsing
    pub from_toml(path: String) -> Result<Manifest> {
        let manifest_path = match Path::new(path).canonicalize() {
            Ok(p) => p,
            Err(e) => bail!(e),
        };

        let manifest_string = read_to_string(manifest_path)?;

        println!("Manifest String: {}", manifest_string);

        let table: Table = match manifest_string.parse::<Table>() {
            Ok(t) => t,
            Err(e) => bail!(e)
        };

        let workspace_table = match table["workspace"].as_table() {
            Some(t) => t,
            None => bail!(ManifestError::MissingTable("workspace"))
        };

        // Validate Rush manifest syntax
        ensure_syntax(
            "Workspace must have a name".to_string(),
            workspace_table.contains_key("name"),
        );

        println!("{}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_toml() {
    }
}

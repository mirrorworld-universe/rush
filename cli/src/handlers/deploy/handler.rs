use crate::{error::*, handlers::CliHandler};
use anyhow::{bail, Result};
use clap::ArgMatches;
use rush_ecs_manifest::{Chain, Manifest};
use rush_ecs_sdk::bevy::BevySDK;
use std::path::Path;

pub struct DeployHandler;

impl CliHandler for DeployHandler {
    // TODO: Validate arguments
    async fn handle_matches(matches: &ArgMatches) -> Result<()> {
        if !Path::new("./Rush.toml").exists() {
            bail!(CliError::NotRushWorkspace)
        }
        if !Path::new("./blueprint/world.toml").exists() {
            bail!(CliError::MissingBlueprint)
        }

        let manifest = Manifest::from_toml("./Rush.toml")?;

        let Chain::Solana {
            store,
            rpc,
            keypair,
        } = manifest.chain;
        let mut sdk = BevySDK::new(rpc, &store, "./blueprint", &keypair);

        sdk.migrate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view() {}
}

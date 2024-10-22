use crate::{error::*, handlers::CliHandler};
use anyhow::{bail, Result};
use clap::ArgMatches;
use rush_parser::{toml::TomlParser, Loader};
use std::path::Path;

pub struct DeployHandler;

impl CliHandler for DeployHandler {
    // TODO: Validate arguments
    fn handle_matches(matches: &ArgMatches) -> Result<()> {
        if !Path::new("./Rush.toml").exists() {
            bail!(CliError::NotRushWorkspace)
        }
        if !Path::new("./blueprint/world.toml").exists() {
            bail!(CliError::MissingBlueprint)
        }

        let loader = Loader::new(TomlParser {});
        let blueprint_path = Path::new("./blueprint").canonicalize()?;
        let blueprint = loader.load_blueprint(&blueprint_path)?;

        println!("{blueprint}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view() {}
}

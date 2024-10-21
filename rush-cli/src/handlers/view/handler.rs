use crate::{error::*, handlers::CliHandler};
use anyhow::{bail, Result};
use clap::ArgMatches;
use rush_parser::{toml::TomlParser, Loader};
use std::path::Path;

pub struct ViewHandler;

impl CliHandler for ViewHandler {
    // TODO: Validate arguments
    // TODO: Make view command work even if not in root folder
    fn handle_matches(_matches: &ArgMatches) -> Result<()> {
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

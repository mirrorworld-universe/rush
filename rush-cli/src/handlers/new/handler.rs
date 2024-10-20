use crate::error::*;
use crate::handlers::CliHandler;
use anyhow::{bail, Result};
use clap::ArgMatches;
use std::path::Path;

pub struct NewHandler;

/// Rush New Command
///
/// # Arguments
///
/// * `--path`, `-p` - PATH of where to store the new project
///
/// # Examples
///
/// ```bash
/// rush new --path path/to/folder
///
/// # Create the workspace in the current directory
/// rush new --path .
/// ```
///
impl CliHandler for NewHandler {
    // TODO: Validate arguments
    fn handle_matches(matches: &ArgMatches) -> Result<()> {
        // Get Name
        let name_value = match matches.get_one::<String>("NAME") {
            Some(n) => n,
            None => bail!(CliError::MissingArgument("NAME".to_string())),
        };

        // Get Path
        let current_dir = String::from(".");
        let path_value = matches.get_one::<String>("PATH").unwrap_or(&current_dir);

        let path = match Path::new(path_value).canonicalize() {
            Ok(p) => p,
            Err(e) => bail!(e),
        };

        Ok(())
    }
}

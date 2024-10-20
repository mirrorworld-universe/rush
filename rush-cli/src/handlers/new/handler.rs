use crate::error::*;
use crate::handlers::CliHandler;
use anyhow::{bail, Result};
use clap::ArgMatches;

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
        let mut is_current_dir = false;

        // Get Name
        let name_value = match matches.get_one::<String>("NAME") {
            Some(n) => n,
            None => bail!(CliError::MissingArgument("NAME".to_string())),
        };

        // Get Path
        let current_dir = String::from(".");
        let path_value = matches.get_one::<String>("PATH").unwrap_or(&current_dir);

        if path_value == current_dir {
            is_current_dir = true;
        }

        if is_current_dir {
            // Doesn't create project folder, just files
        } else {
            // Creates project folder and files
        }

        Ok(())
    }
}

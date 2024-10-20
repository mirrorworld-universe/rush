use crate::handlers::CliHandler;
use anyhow::Result;
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
    fn handle_matches(matches: &ArgMatches) -> Result<()> {
        let current_dir = String::from(".");
        let value = matches.get_one::<String>("PATH").unwrap_or(&current_dir);
        let path = Path::new(value).canonicalize().unwrap();

        println!("{}", path.display());

        Ok(())
    }
}

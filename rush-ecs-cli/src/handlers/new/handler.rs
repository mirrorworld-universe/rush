use crate::{error::*, handlers::CliHandler, utils::print_happy_peepo};
use anyhow::{bail, Result};
use clap::ArgMatches;
use colored::Colorize;
use rush_manifest::{Chain, Manifest};
use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

pub struct NewHandler;

/// Rush New Command
///
/// # Arguments
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
    async fn handle_matches(matches: &ArgMatches) -> Result<()> {
        // Get Name
        let name_value = match matches.get_one::<String>("NAME") {
            Some(n) => n,
            None => bail!(CliError::MissingArgument("NAME".to_string())),
        };

        // Get Path
        let current_dir = String::from(".");
        let path_value = matches.get_one::<String>("PATH").unwrap_or(&current_dir);

        let mut manifest = Manifest::new_solana(name_value.to_string());
        manifest.chain = Chain::Solana {
            store: "<STORAGE_PROGRAM_ADDRESS>".to_string(),
            rpc: "<HTTPS_RPC_URL>".to_string(),
            keypair: "<KEYPAIR_PATH>".to_string(),
        };

        // Creates project folder and files
        let project_path = Path::new(path_value).canonicalize()?;
        let folder_path = project_path.join(name_value);

        // TODO: (Review) Consider using <AsRef<Path>> for path args
        let folder_path_str = folder_path.clone();
        let folder_path_str = match folder_path_str.to_str() {
            Some(p) => p,
            None => bail!(CliError::InvalidPathConversion),
        };

        create_dir(folder_path)?;
        create_project_files(manifest, folder_path_str)?;

        print_happy_peepo();
        println!("[{}] Rush project created.", "SUCCESS".green().bold());

        Ok(())
    }
}

// TODO: (Review) Consider attaching this function to `rush-core::Blueprint`
/// Create blueprint directory and files
fn create_blueprint_files(path: &str, folder_name: &str, filename: &str) -> Result<()> {
    let project_path = Path::new(path).canonicalize()?;
    let folder_path = project_path.join(folder_name);
    let file_path = folder_path.join(filename);

    // create blueprint folder
    create_dir(folder_path)?;

    // create default blueprint file
    let mut blueprint_file = File::create(file_path)?;
    blueprint_file
        .write_all(b"[world]\nname = \"My Onchain World\"\ndescription = \"My Onchain World Description\"\nentities = [\"player\"]\nregions = [\"base\"]\n\n[entity]\nplayer = { name = \"String\", x =\"f64\", y = \"f64\"}\n\n[base]")?;

    Ok(())
}

/// Create project files
///
/// # Examples
///
/// Project Structure
///
/// ```
/// project/
///     Rush.toml
///     blueprint/
///         world.toml
/// ```
///
/// # Failures
///
/// This function fails if:
/// 1. The path is `.` (current directory) and it's not empty
///
fn create_project_files(manifest: Manifest, path: &str) -> Result<()> {
    Manifest::save_toml(manifest, path)?;
    create_blueprint_files(path, "blueprint", "world.toml")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {}
}

mod error;
mod handlers;
mod utils;

use anyhow::Result;
use clap::{Arg, Command};
use handlers::{CliHandler, NewHandler};

fn main() -> Result<()> {
    /*
     * @dev We'll leave the matchers here despite it being bulky to have a quick
     * single point overview of all the commands and subcommands available
     */
    let top_level_matches = Command::new("rush")
        // .version("0.1.0") // commented out to get version from manifest
        .about("Rapid and Declarative development framework for Fully Onchain Games (FOCG) and Autonomous Worlds (AW) by SonicSVM.")
        .arg_required_else_help(true) .subcommand(
            Command::new("new")
                .about("Create a new Rush project.")
                .arg(Arg::new("NAME").help("Project name").required(true))
                .arg(Arg::new("PATH").help("Project path.").long("path").short('p'))
        )
        .subcommand(
            Command::new("deploy")
                .about("Deploy current Rush project")
                .arg_required_else_help(true)
                .arg(Arg::new("PATH").help("Path of the Rush project.").long("path").short('p'))
                .arg(Arg::new("DRY-RUN").help("Parse the current Gaming Blueprint without deploying. Useful for catching predeploy bugs.").long("dry-run").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("view")
                .about("Create a new Rush project.")
        )
        .subcommand(
            Command::new("config")
                .about("Change or view your current Rush CLI configurations.")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get").about("Get your current Rush CLI configurations.")
                )
                .subcommand(
                    Command::new("set")
                        .about("Get your current Rush CLI configurations.")
                        .subcommand(
                            Command::new("rpc").about("Change your RPC URL.")
                            .arg(Arg::new("URL").required(true).long("url").short('u'))
                        )
                        .subcommand(
                            Command::new("ws").about("Change your Websockets URL.")
                            .arg(Arg::new("URL").required(true).long("url").short('u'))
                        )
                        .subcommand(
                            Command::new("keypair").about("Change your keypair path.")
                            .arg(Arg::new("PATH").required(true).long("path").short('p'))
                        )
                        .subcommand(
                            Command::new("blueprint").about("Change your Blueprint path. Can be a FILE or DIRECTORY.")
                            .arg(Arg::new("PATH").required(true).long("path").short('p'))
                        )
                )
        )
        .get_matches();

    match top_level_matches.subcommand() {
        Some(("new", sub_matches)) => NewHandler::handle_matches(sub_matches),

        // Some(("deploy", sub_matches)) => {}
        //
        // Some(("view", sub_matches)) => {}
        //
        // Some(("config", sub_matches)) => {}

        // impossible to reach due to arg_required_else_help()
        _ => Ok(()),
    }
}

use anyhow::Result;
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("rush")
        // .version("0.1.0") // get version from manifest
        .about("Rapid and Declarative development framework for Fully Onchain Games (FOCG) and Autonomous Worlds (AW) by SonicSVM.")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("deploy")
                .about("Deploy current Rush project")
                .arg_required_else_help(true)
                .arg(Arg::new("PATH").required(false).long("path").short('p'))
        )
        .subcommand(
            Command::new("new")
                .about("Create a new Rush project.")
                .arg_required_else_help(true)
                .arg(Arg::new("PATH").required(false).long("path").short('p'))
                .arg(Arg::new("DRY-RUN").long("dry-run").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("storage")
                .about("Interact with your Rush project's storage.")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("start").about("Start local storage for local testing.")
                )
                .subcommand(
                    Command::new("reset").about("Reset your current storage.")
                )
                .subcommand(
                    Command::new("view").about("View the state of your current Rush project.")
                    .subcommand(
                        Command::new("blueprint").about("View the Blueprint of your current Rush project.")
                    )
                    .subcommand(
                        Command::new("world")
                            .about("View the World table of your current Rush project.")
                            .arg(Arg::new("ADDRESS").required(true).long("address").short('a'))
                    )
                    .subcommand(
                        Command::new("entity")
                            .about("View the Entity properties table of a specific entity in your current Rush project.")
                            .arg(Arg::new("NAME").required(true).long("name").short('n'))
                    )
                    .subcommand(
                        Command::new("instance")
                            .about("View the data of a specified instance.")
                            .arg(Arg::new("WORLD").required(true).long("world").short('w'))
                            .arg(Arg::new("REGION").required(true).long("region").short('r'))
                            .arg(Arg::new("ENTITY").required(true).long("entity").short('e'))
                    )
                )
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
                            .arg(Arg::new("MONIKER_OR_URL").required(true).long("url").short('u'))
                        )
                        .subcommand(
                            Command::new("ws").about("Change your Websockets URL.")
                            .arg(Arg::new("MONIKER_OR_URL").required(true).long("url").short('u'))
                        )
                        .subcommand(
                            Command::new("keypair").about("Change your keypair path.")
                            .arg(Arg::new("PATH").required(true).long("path").short('p'))
                        )
                        .subcommand(
                            Command::new("blueprint").about("Change your Blueprint path.")
                            .arg(Arg::new("FILE_OR_DIRECTORY").required(true).long("path").short('p'))
                        )
                        .subcommand(
                            Command::new("storage").about("Change your current storage for local testing.")
                            .arg(Arg::new("<STORAGE_OPTION>").required(true).long("storage").short('s'))
                        )
                )
        )
        .get_matches();

    Ok(())
}

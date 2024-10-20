use anyhow::Result;
use clap::ArgMatches;

pub trait CliHandler {
    fn handle_matches(matches: &ArgMatches) -> Result<()>;
}

use anyhow::Result;
use clap::ArgMatches;

pub trait CliHandler {
    async fn handle_matches(matches: &ArgMatches) -> Result<()>;
}

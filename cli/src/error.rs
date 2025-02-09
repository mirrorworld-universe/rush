use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("expected argument: {0}")]
    MissingArgument(String),
    #[error("error converting path to string")]
    InvalidPathConversion,
    #[error("can't find Blueprint")]
    MissingBlueprint,
    #[error("not in a Rush workspace")]
    NotRushWorkspace,
}

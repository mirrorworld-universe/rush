use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Expected argument: {0}")]
    MissingArgument(String),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("expected argument: {0}")]
    MissingArgument(String),
}

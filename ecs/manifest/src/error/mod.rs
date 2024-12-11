pub mod utils;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("expected table: {0}")]
    MissingTable(String),
    #[error("unsupported repository: {0}")]
    UnsupportedRepo(String),
}

pub mod utils;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Expected table: {0}")]
    MissingTable(String),
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Expected table: {0}")]
    MissingTable(String),
}

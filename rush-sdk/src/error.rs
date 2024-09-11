use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Storage not yet migrated")]
    NotYetMigrated,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Keypair not found in path")]
    KeypairNotFound(String),

    #[error("Sign in to authenticate")]
    Unauthenticated,
}

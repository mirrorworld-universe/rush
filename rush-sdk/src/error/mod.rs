use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("storage not yet migrated")]
    NotMigrated,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("keypair not found in path")]
    KeypairNotFound(String),

    #[error("sign in to authenticate")]
    Unauthenticated,
}

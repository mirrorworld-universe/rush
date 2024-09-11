use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Region not found in World")]
    RegionNotFound,

    #[error("Entity not found in World")]
    EntityNotFound,

    #[error("Instance not found in World")]
    InstanceNotFound,

    #[error("Component not in Entity's tree")]
    ComponentNotFound,

    #[error("Cannot set a different data type")]
    MismatchedDataType,

    #[error("Unsupported component data type")]
    UnsupportedDataType,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("region not found in world")]
    RegionNotFound,

    #[error("entity not found in world")]
    EntityNotFound,

    #[error("instance not found in world")]
    InstanceNotFound,

    #[error("component not in entity's tree")]
    ComponentNotFound,

    #[error("cannot set a different data type")]
    MismatchedDataType,

    #[error("unsupported component data type")]
    UnsupportedDataType,
}

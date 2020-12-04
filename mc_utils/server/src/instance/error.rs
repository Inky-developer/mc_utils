use std::fmt::Display;

pub type Result<T> = std::result::Result<T, InstanceError>;

#[derive(Debug)]
pub enum InstanceError {
    Other(String),
    IoError(std::io::Error),
    DeserializationError(java_properties::PropertiesError),
}

impl From<std::io::Error> for InstanceError {
    fn from(error: std::io::Error) -> Self {
        InstanceError::IoError(error)
    }
}

impl From<java_properties::PropertiesError> for InstanceError {
    fn from(error: java_properties::PropertiesError) -> Self {
        InstanceError::DeserializationError(error)
    }
}

impl Display for InstanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstanceError::Other(msg) => f.write_str(&msg),
            InstanceError::IoError(error) => error.fmt(f),
            InstanceError::DeserializationError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for InstanceError {}

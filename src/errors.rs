use thiserror::Error;

/// all possible errors for 'sec' and 'par' commands processing
#[derive(Debug, Error)]
pub enum SecParError {
    /// cannot get secret
    #[error("Secret/Parameter Not Found: {0}")]
    NotFound(String),
    /// cannot create secret/parameter
    #[error("Failed to create secret/parameter: {0}")]
    CreateFail(String),
    /// unable to parse spec file
    #[error("Failed to parse spec file")]
    InvalidSpec(#[from] ParseFileError),
}

/// An enumeration of possible errors when parsing a yaml file
#[derive(Debug, Error)]
pub enum ParseFileError {
    /// Error deserializing extensions
    #[error("Path: `{0}`")]
    NoSuchFile(String, #[source] std::io::Error),
    /// Error deserializing extensions
    #[error("Path: `{0}`")]
    DeserializeError(String, #[source] serde_yaml::Error),
}

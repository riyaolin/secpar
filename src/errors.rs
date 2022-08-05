use thiserror::Error;

/// all possible errors
#[derive(Debug, Error)]
pub enum SecParError {
    /// cannot get secret
    #[error("Secret/Parameter Not Found: {0}")]
    NotFound(String),
    /// cannot create secret/parameter
    #[error("Failed to create secret/parameter: {0}")]
    CreateFail(String),
}

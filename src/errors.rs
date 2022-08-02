use thiserror::Error;

/// all possible errors
#[derive(Debug, Error)]
pub enum SecParError {
    /// cannot get secret
    #[error("Failed to get secret: {0}")]
    NotFound(String),
    /// cannot create secret
    #[error("Failed to create secret: {0}")]
    CreateFail(String),
}

use std::error::Error;
use thiserror::Error;

/// Builds a detailed error message by walking the full [`Error::source`] chain.
///
/// `e.to_string()` on an AWS `SdkError` only returns the top-level category
/// (e.g. `"service error"`).  This function appends every cause in the chain
/// so the message shown to the user includes the actual AWS reason
/// (e.g. `"service error: InvalidRequestException: You can't create this secret…"`).
pub fn format_sdk_error(e: &dyn Error) -> String {
    let mut msg = e.to_string();
    let mut source = e.source();
    while let Some(cause) = source {
        msg.push_str(": ");
        msg.push_str(&cause.to_string());
        source = cause.source();
    }
    msg
}

/// All possible errors produced by the `sec` and `par` subcommands.
///
/// This enum is used as the error type throughout the CLI command handlers.
/// AWS-level failures surface as [`SecParError::AwsSdk`]; missing response
/// fields produce [`SecParError::MissingValue`]; YAML spec problems become
/// [`SecParError::InvalidSpec`]; and terminal interaction failures become
/// [`SecParError::Interactive`].
#[derive(Debug, Error)]
pub enum SecParError {
    /// Produced when an AWS API call returns a "resource not found" condition
    /// (e.g., requesting a secret or parameter that does not exist).
    #[error("Secret/Parameter Not Found: {0}")]
    NotFound(String),

    /// Produced when an AWS create/put operation fails (e.g., name collision
    /// or insufficient IAM permissions).
    #[error("Failed to create secret/parameter: {0}")]
    CreateFail(String),

    /// Produced when the underlying AWS SDK returns any error not covered by a
    /// more specific variant.  The inner `String` is the SDK's `Display`
    /// representation.
    #[error("AWS SDK error: {0}")]
    AwsSdk(String),

    /// Produced when an AWS response is structurally valid but a required field
    /// (e.g., `secret_string`, `arn`, or `parameter.value`) is absent.
    #[error("Missing value: {0}")]
    MissingValue(String),

    /// Produced when a YAML spec file cannot be opened or deserialized.
    /// Wraps [`ParseFileError`] via `#[from]`.
    #[error("Failed to parse spec file")]
    InvalidSpec(#[from] ParseFileError),

    /// Produced when a terminal interactive prompt (spinner, confirmation,
    /// selection menu) fails — typically because stdin is not a TTY.
    #[error("Interactive prompt error: {0}")]
    Interactive(String),
}

/// Errors that can occur while reading and parsing a YAML spec file.
#[derive(Debug, Error)]
pub enum ParseFileError {
    /// The file at the given path could not be opened (e.g., does not exist or
    /// insufficient permissions).  The inner source is the originating
    /// [`std::io::Error`].
    #[error("Path: `{0}`")]
    NoSuchFile(String, #[source] std::io::Error),

    /// The file was opened but its contents could not be deserialized as the
    /// expected type.  The inner source is the originating
    /// [`serde_norway::Error`].
    #[error("Path: `{0}`")]
    DeserializeError(String, #[source] serde_norway::Error),
}

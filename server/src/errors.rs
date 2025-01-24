#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("violated validation")]
    Validation,
    #[error("failed to parse request")]
    InvalidFormat,
    #[error("Invalid request format")]
    UnknownFormat,
}

#[derive(Debug, thiserror::Error)]
#[error("An unrecoverable fatal error occurred.")]
pub struct UnrecoverableError;

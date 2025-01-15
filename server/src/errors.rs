#[derive(Debug, thiserror::Error)]
pub enum ServerError {}

#[derive(Debug, thiserror::Error)]
#[error("An unrecoverable fatal error occurred.")]
pub struct UnrecoverableError;

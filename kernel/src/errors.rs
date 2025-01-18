#[derive(Debug, thiserror::Error)]
#[error("input cannot be converted.")]
pub struct FormationError;

#[derive(Debug, thiserror::Error)]
#[error("violation of validation rules")]
pub struct ValidationError;

#[derive(Debug, thiserror::Error)]
#[error("driver error")]
pub struct DriverError;

#[derive(Debug, thiserror::Error)]
#[error("failed to initialize database")]
pub struct FailedInitializeDataBase;


#[derive(Debug, thiserror::Error)]
#[error("Failed to build a read model.")]
pub struct FailedBuildReadModel;

#[cfg(test)]
pub(crate) mod test {
    #[derive(Debug, thiserror::Error)]
    #[error("Unrecoverable error")]
    pub struct UnrecoverableError;
}
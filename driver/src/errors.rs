#[derive(Debug, thiserror::Error)]
#[error("failed to initialize database")]
pub struct FailedInitializeDataBase;


#[derive(Debug, thiserror::Error)]
#[error("Failed to build a read model.")]
pub struct FailedBuildReadModel;

#[derive(Debug, thiserror::Error)]
#[error("Failed to query the database.")]
pub struct FailedQuery;

#[cfg(test)]
pub(crate) mod test {
    #[derive(Debug, thiserror::Error)]
    #[error("Unrecoverable error")]
    pub struct UnrecoverableError;
}
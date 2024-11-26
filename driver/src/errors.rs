#[derive(Debug, thiserror::Error)]
pub enum DriverError {
    #[error("Cannot access database to {0}")]
    Connection(String),
    #[error("Failure migration database")]
    Migration,
    #[error("Cannot setup event store")]
    SetupEventStore,
}

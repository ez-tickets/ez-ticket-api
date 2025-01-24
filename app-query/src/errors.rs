#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("An error occurred while querying the database")]
    Driver
}
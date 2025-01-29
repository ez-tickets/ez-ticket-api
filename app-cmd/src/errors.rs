#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("The resource could not be found or the `Projection` may have failed.")]
    Formation,
    
    #[error("Failed to spawn process")]
    Process,

    #[error("Request requires an identifier")]
    RequiredId,
    
    #[error("Cannot find resource")]
    NotFound,
    
    #[error("An error occurred due to kernel module")]
    Kernel,
    
    #[error("Invalid command")]
    InvalidCommand,
}
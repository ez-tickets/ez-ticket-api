#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Failed to convert request from server to command")]
    Formation,
    
    #[error("Failed to spawn process")]
    Process,

    #[error("Request requires an identifier")]
    RequiredId,
    
    #[error("Cannot find resource")]
    NotFound,
    
    #[error("An error occurred due to kernel module")]
    Kernel,
}
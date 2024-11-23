#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("duplicate data in resource ({entity}): {id}.")]
    AlreadyExists {
        entity: &'static str,
        id: String
    },
    #[error("not found data in resource ({entity}): {id}.")]
    NotFound {
        entity: &'static str,
        id: String,
    },
    #[error("Something went wrong...")]
    Invalid,
}

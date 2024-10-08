use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
}


#[derive(Debug)]
pub struct UnrecoverableError;

impl Display for UnrecoverableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An unrecoverable fatal error occurred.")
    }
}

impl Context for UnrecoverableError {}
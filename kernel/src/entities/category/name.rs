use crate::errors::ValidationError;
use error_stack::Report;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CategoryName(String);

impl CategoryName {
    pub fn new(name: impl Into<String>) -> Result<CategoryName, Report<ValidationError>> {
        let name = name.into();

        if name.is_empty() {
            return Err(
                Report::new(ValidationError).attach_printable("`CategoryName` must not be empty")
            );
        }

        Ok(Self(name))
    }
}

impl AsRef<str> for CategoryName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<CategoryName> for String {
    fn from(name: CategoryName) -> Self {
        name.0
    }
}

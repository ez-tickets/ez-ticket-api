use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryName(String);

impl CategoryName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
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

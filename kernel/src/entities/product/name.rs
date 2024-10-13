use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(name: impl Into<String>) -> ProductName {
        Self(name.into())
    }
}

impl AsRef<str> for ProductName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductName> for String {
    fn from(value: ProductName) -> Self {
        value.0
    }
}

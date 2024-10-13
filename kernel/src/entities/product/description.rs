use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductDescription(String);

impl ProductDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self(description.into())
    }
}

impl AsRef<str> for ProductDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductDescription> for String {
    fn from(value: ProductDescription) -> Self {
        value.0
    }
}

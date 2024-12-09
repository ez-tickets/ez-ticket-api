use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct CatalogName(String);

impl CatalogName {
    pub fn new(name: impl Into<String>) -> CatalogName {
        Self(name.into())
    }
}

impl AsRef<str> for CatalogName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<CatalogName> for String {
    fn from(value: CatalogName) -> Self {
        value.0
    }
}

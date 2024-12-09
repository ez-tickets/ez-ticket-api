use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogDesc(String);

impl CatalogDesc {
    pub fn new(desc: impl Into<String>) -> CatalogDesc {
        Self(desc.into())
    }
}

impl AsRef<str> for CatalogDesc {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<CatalogDesc> for String {
    fn from(value: CatalogDesc) -> Self {
        value.0
    }
}

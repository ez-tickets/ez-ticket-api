use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ProductDesc(String);

impl ProductDesc {
    pub fn new(desc: impl Into<String>) -> Self {
        Self(desc.into())
    }
}

impl AsRef<str> for ProductDesc {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<ProductDesc> for String {
    fn from(desc: ProductDesc) -> Self {
        desc.0
    }
}

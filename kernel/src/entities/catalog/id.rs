use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct CatalogId(Uuid);

impl CatalogId {
    pub fn new(id: impl Into<Uuid>) -> CatalogId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for CatalogId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<CatalogId> for Uuid {
    fn from(value: CatalogId) -> Self {
        value.0
    }
}

impl Display for CatalogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CatalogId({})", self.0)
    }
}

impl Default for CatalogId {
    fn default() -> Self {
        CatalogId(Uuid::new_v4())
    }
}

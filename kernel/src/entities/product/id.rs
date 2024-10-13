use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new(id: impl Into<Uuid>) -> ProductId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for ProductId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<ProductId> for Uuid {
    fn from(value: ProductId) -> Self {
        value.0
    }
}

impl Display for ProductId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProductId({})", self.0)
    }
}

impl Default for ProductId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

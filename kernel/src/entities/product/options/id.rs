use crate::entities::ProductId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ProductOptionId(Uuid);

impl ProductOptionId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl AsRef<Uuid> for ProductOptionId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<ProductOptionId> for Uuid {
    fn from(value: ProductOptionId) -> Self {
        value.0
    }
}

impl From<ProductOptionId> for ProductId {
    fn from(value: ProductOptionId) -> Self {
        Self::new(value.0)
    }
}

impl Default for ProductOptionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

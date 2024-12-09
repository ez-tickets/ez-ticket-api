use crate::entities::ProductId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct OptionId(Uuid);

impl OptionId {
    pub fn new(id: impl Into<Uuid>) -> OptionId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for OptionId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<OptionId> for Uuid {
    fn from(value: OptionId) -> Self {
        value.0
    }
}

impl From<OptionId> for ProductId {
    fn from(value: OptionId) -> Self {
        ProductId::new(value)
    }
}

impl From<ProductId> for OptionId {
    fn from(value: ProductId) -> Self {
        OptionId::new(value)
    }
}

impl Display for OptionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionId({})", self.0)
    }
}
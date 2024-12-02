use std::fmt::{Display, Formatter};
use crate::entities::ProductId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BindId(Uuid);

impl From<BindId> for Uuid {
    fn from(value: BindId) -> Self {
        value.0
    }
}

impl AsRef<Uuid> for BindId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Display for BindId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bind({})", self.0)
    }
}

impl Default for BindId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<ProductId> for BindId {
    fn from(value: ProductId) -> Self {
        Self(value.into())
    }
}

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new(id: impl Into<Uuid>) -> OrderId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for OrderId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<OrderId> for Uuid {
    fn from(value: OrderId) -> Self {
        value.0
    }
}

impl Display for OrderId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OrderId({})", self.0)
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
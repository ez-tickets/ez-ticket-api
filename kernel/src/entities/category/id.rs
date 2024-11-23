use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct CategoryId(Uuid);

impl CategoryId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl AsRef<Uuid> for CategoryId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<CategoryId> for Uuid {
    fn from(id: CategoryId) -> Self {
        id.0
    }
}

impl Default for CategoryId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for CategoryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CategoryId({})", self.0)
    }
}
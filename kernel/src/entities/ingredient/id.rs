use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct IngredientId(Uuid);

impl IngredientId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl AsRef<Uuid> for IngredientId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<IngredientId> for Uuid {
    fn from(id: IngredientId) -> Self {
        id.0
    }
}

impl Default for IngredientId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

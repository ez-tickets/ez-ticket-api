use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct RecipeId(Uuid);

impl RecipeId {
    pub fn new(id: impl Into<Uuid>) -> RecipeId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for RecipeId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<RecipeId> for Uuid {
    fn from(value: RecipeId) -> Self {
        value.0
    }
}

impl Default for RecipeId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngredientName(String);

impl IngredientName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl AsRef<str> for IngredientName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<IngredientName> for String {
    fn from(name: IngredientName) -> Self {
        name.0
    }
}

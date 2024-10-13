mod id;

pub use self::id::*;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entities::{IngredientId, ProductId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Recipe {
    id: RecipeId,
    product: ProductId,
    ingredients: HashSet<IngredientId>,
}

impl Recipe {
    pub fn new(
        id: RecipeId,
        product: ProductId,
        ingredients: impl IntoIterator<Item = IngredientId>,
    ) -> Recipe {
        Self {
            id,
            product,
            ingredients: ingredients.into_iter().collect(),
        }
    }
}

impl Recipe {
    pub fn id(&self) -> &RecipeId {
        &self.id
    }
}

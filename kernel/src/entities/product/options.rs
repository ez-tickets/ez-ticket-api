mod id;

pub use self::id::ProductOptionId;

use crate::entities::{IngredientId, Price, ProductName};
use crate::errors::KernelError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductOption {
    id: ProductOptionId,
    name: ProductName,
    price: Price,
}

impl ProductOption {
    pub fn create(
        id: ProductOptionId,
        name: ProductName,
        price: Price,
        ingredient: HashSet<IngredientId>,
        ingredient_consume: impl Fn(&HashSet<IngredientId>) -> Result<(), KernelError>,
    ) -> Result<Self, KernelError> {
        ingredient_consume(&ingredient)?;
        Ok(Self { id, name, price })
    }
}

impl Eq for ProductOption {}

impl PartialEq<Self> for ProductOption {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Hash for ProductOption {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.price.hash(state);
    }
}

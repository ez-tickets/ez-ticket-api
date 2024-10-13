mod amount;
mod id;
mod name;

pub use self::amount::*;
pub use self::id::*;
pub use self::name::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ingredient {
    id: IngredientId,
    name: IngredientName,
    amount: Amount,
}

impl Ingredient {
    pub fn new(id: IngredientId, name: IngredientName, amount: Amount) -> Self {
        Self { id, name, amount }
    }
}

impl Ingredient {
    pub fn id(&self) -> &IngredientId {
        &self.id
    }

    pub fn name(&self) -> &IngredientName {
        &self.name
    }

    pub fn amount(&self) -> &Amount {
        &self.amount
    }
}

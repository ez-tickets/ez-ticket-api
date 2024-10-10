use crate::entities::{ProductOptionId, Quantity};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderOption {
    id: ProductOptionId,
    quantity: Quantity,
}

impl OrderOption {
    pub fn new(id: ProductOptionId, quantity: Quantity) -> Self {
        Self { id, quantity }
    }
}

impl OrderOption {
    pub fn id(&self) -> &ProductOptionId {
        &self.id
    }

    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }
}

impl Eq for OrderOption {}

impl PartialEq for OrderOption {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for OrderOption {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.quantity.hash(state);
    }
}

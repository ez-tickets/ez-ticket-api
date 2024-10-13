use crate::entities::{ProductId, Quantity};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderProduct {
    id: ProductId,
    quantity: Quantity,
}

impl OrderProduct {
    pub fn new(id: ProductId, quantity: Quantity) -> Self {
        Self { id, quantity }
    }
}

impl OrderProduct {
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }
}

impl Eq for OrderProduct {}

impl PartialEq for OrderProduct {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for OrderProduct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.quantity.hash(state);
    }
}

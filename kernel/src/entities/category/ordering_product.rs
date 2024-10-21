use crate::entities::ProductId;
use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct OrderingProduct {
    ordered: i32,
    product_id: ProductId,
}

impl OrderingProduct {
    pub fn new(ordered: i32, product_id: ProductId) -> Self {
        Self {
            ordered,
            product_id,
        }
    }

    pub fn ordered(&self) -> i32 {
        self.ordered
    }

    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }
}

impl Eq for OrderingProduct {}

impl PartialEq<Self> for OrderingProduct {
    fn eq(&self, other: &Self) -> bool {
        self.ordered.eq(&other.ordered) && self.product_id.eq(&other.product_id)
    }
}

impl PartialOrd<Self> for OrderingProduct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderingProduct {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordered.cmp(&other.ordered)
    }
}

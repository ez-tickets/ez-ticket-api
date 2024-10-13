mod id;

pub use self::id::*;

use crate::entities::{Price, ProductId, ProductName};
use crate::errors::KernelError;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductOption {
    id: ProductOptionId,
    product: ProductId,
    name: ProductName,
    price: Price,
}

impl ProductOption {
    pub fn new(
        id: ProductOptionId,
        product: ProductId,
        name: ProductName,
        price: Price,
    ) -> Result<Self, KernelError> {
        Ok(Self { id, product, name, price })
    }
}

impl Eq for ProductOption {}

impl PartialEq<Self> for ProductOption {
    fn eq(&self, other: &Self) -> bool {
        self.product.eq(&other.product)
    }
}

impl Hash for ProductOption {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.product.hash(state);
        self.name.hash(state);
        self.price.hash(state);
    }
}

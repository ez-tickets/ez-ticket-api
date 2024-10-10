use crate::entities::{Price, ProductId, ProductName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductOption {
    id: ProductId,
    name: ProductName,
    price: Price,
}

impl ProductOption {
    pub fn new(id: ProductId, name: ProductName, price: Price) -> Self {
        Self { id, name, price }
    }
}

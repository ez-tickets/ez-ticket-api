use crate::entities::{OrderOption, OrderProduct};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderItem {
    product: OrderProduct,
    product_option: Option<HashSet<OrderOption>>,
}

impl OrderItem {
    pub fn new(product: OrderProduct, product_option: Option<HashSet<OrderOption>>) -> Self {
        Self {
            product,
            product_option,
        }
    }
}

impl OrderItem {
    pub fn product(&self) -> &OrderProduct {
        &self.product
    }

    pub fn product_option(&self) -> &Option<HashSet<OrderOption>> {
        &self.product_option
    }
}

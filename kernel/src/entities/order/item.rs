use crate::entities::{ProductId, Quantity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderItem {
    product: ProductId,
    quantity: Quantity,
}

use serde::{Deserialize, Serialize};
use crate::entities::ProductId;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductOption {
    id: ProductId
}
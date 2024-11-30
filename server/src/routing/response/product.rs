use serde::Serialize;
use kernel::entities::ProductId;

#[derive(Serialize)]
pub struct AllProductIdResponse {
    pub products: Vec<ProductId>
}


use crate::entities::ProductId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MainProduct(ProductId);

impl MainProduct {
    pub fn new(id: ProductId) -> Self {
        Self(id)
    }
}

impl AsRef<ProductId> for MainProduct {
    fn as_ref(&self) -> &ProductId {
        &self.0
    }
}

impl From<MainProduct> for ProductId {
    fn from(main_product: MainProduct) -> Self {
        main_product.0
    }
}

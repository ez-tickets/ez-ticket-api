use kernel::entities::{CategoryId, ProductId};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
pub struct ProductFilter {
    pub category: Option<CategoryId>
}

#[derive(Default)]
pub struct RegisterProduct {
    pub name: String,
    pub image: Vec<u8>
}

impl Debug for RegisterProduct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterProduct")
            .field("name", &self.name)
            .field("image", &"<byte-data>")
            .finish()
    }
}

#[derive(Deserialize)]
pub struct FindByProductId {
    pub id: ProductId
}

#[derive(Deserialize)]
pub struct UpdateProductName {
    pub name: String
}
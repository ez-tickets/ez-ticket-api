use std::fmt::{Debug, Formatter};
use kernel::entities::{CategoryId, ImageId};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProductFilter {
    pub category: Option<CategoryId>
}

#[derive(Default)]
pub struct RegisterProduct {
    pub name: String,
    pub desc: String,
    pub price: i32,
    pub image: Vec<u8>
}

impl Debug for RegisterProduct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegisterProduct")
            .field("name", &self.name)
            .field("desc", &self.desc)
            .field("price", &self.price)
            .field("image", &"<byte-data>")
            .finish()
    }
}
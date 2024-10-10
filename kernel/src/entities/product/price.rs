use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Price(i32);

impl Price {
    pub fn new(price: impl Into<i32>) -> Self {
        Self(price.into())
    }
}

impl AsRef<i32> for Price {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Price> for i32 {
    fn from(price: Price) -> Self {
        price.0
    }
}

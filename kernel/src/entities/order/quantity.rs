use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Quantity(i32);

impl Quantity {
    pub fn new(quantity: impl Into<i32>) -> Self {
        Self(quantity.into())
    }
}

impl AsRef<i32> for Quantity {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Quantity> for i32 {
    fn from(quantity: Quantity) -> Self {
        quantity.0
    }
}

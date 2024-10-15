use crate::entities::Quantity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Stock(i32);

impl Stock {
    pub fn new(stock: impl Into<i32>) -> Self {
        Self(stock.into())
    }

    pub fn in_stock(&mut self, stock: impl Into<i32>) {
        self.0 += stock.into();
    }

    pub fn bring_out(&mut self, out: impl Into<i32>) {
        self.0 -= out.into();
    }
}

impl AsRef<i32> for Stock {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Stock> for i32 {
    fn from(stock: Stock) -> Self {
        stock.0
    }
}

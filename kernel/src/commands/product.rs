use nitinol::Command;
use serde::{Deserialize, Serialize};
use crate::entities::{Price, ProductDescription, ProductName};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductCommand {
    Create {
        name: ProductName,
        desc: ProductDescription,
        price: Price,
    },
    UpdateName {
        name: String,
    },
    UpdateDescription {
        desc: String,
    },
    StockIn {
        amount: i32,
    },
    StockOut {
        amount: i32,
    },
    UpdatePrice {
        price: i32,
    },
    Delete,
}

impl Command for ProductCommand {}
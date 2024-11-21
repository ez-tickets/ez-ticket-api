use nitinol::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductCommand {
    Register {
        name: String,
        desc: String,
        price: i32,
        stock: i32,
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
use nitinol::Command;
use serde::{Deserialize, Serialize};
use crate::entities::{ProductName};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductCommand {
    Register {
        name: ProductName,
    },
    UpdateName {
        name: String,
    },
    Delete,
}

impl Command for ProductCommand {}
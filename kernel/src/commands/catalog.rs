use crate::entities::{CatalogDesc, CatalogName, MainProduct, Price};
use nitinol::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CatalogCommand {
    Create {
        name: CatalogName,
        desc: CatalogDesc,
        price: Price,
        main: MainProduct
    },
    UpdateName {
        name: CatalogName,
    },
    UpdateDesc {
        desc: CatalogDesc,
    },
    UpdatePrice {
        price: Price,
    },
    Delete,
}

impl Command for CatalogCommand {}
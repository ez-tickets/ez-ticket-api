use crate::entities::{CatalogDesc, CatalogName, MainProduct, OptProduct, OptionId, Price, ProductId, SubProduct};
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
    AddMainProd {
        ordering: i32,
        main: ProductId
    },
    UpdateMainProdOrdering {
        ordering: MainProduct
    },
    RemoveMainProd {
        main: ProductId
    },
    AddSubProd {
        ordering: i32,
        sub: ProductId
    },
    UpdateSubProdOrdering {
        ordering: SubProduct
    },
    RemoveSubProd {
        sub: ProductId
    },
    AddOptProd {
        ordering: i32,
        opt: OptionId
    },
    UpdateOptProdOrdering {
        ordering: OptProduct
    },
    RemoveOptProd {
        opt: OptionId
    }
}

impl Command for CatalogCommand {}
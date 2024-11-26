use crate::entities::{CategoryId, CategoryName, ProductId};
use nitinol::Command;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum CategoryCommand {
    Create { name: CategoryName },
    UpdateName { name: String },
    Delete,
    AddProduct { product_id: ProductId },
    UpdateProductOrdering { ordering: BTreeMap<i32, ProductId> },
    RemoveProduct { product: ProductId }
}


impl Command for CategoryCommand {}

#[derive(Debug, Clone)]
pub enum CategoriesCommand {
    Add {
        id: CategoryId
    },
    Remove {
        id: CategoryId
    },
    Update {
        new: BTreeMap<i32, CategoryId>
    }
}

impl Command for CategoriesCommand {}
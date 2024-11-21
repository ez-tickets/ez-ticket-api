use nitinol::Command;
use std::collections::BTreeMap;
use uuid::Uuid;
use crate::entities::CategoryId;

#[derive(Debug, Clone)]
pub enum CategoryCommand {
    Create { name: String, ordering: i32 },
    UpdateName { name: String },
    UpdateOrdering { ordering: i32 },
    Delete,
    AddProduct { ordered: i32, product_id: Uuid },
}


impl Command for CategoryCommand {}

#[derive(Debug, Clone)]
pub struct UpdateCategoryOrderingCommand {
    pub new: BTreeMap<i32, Uuid>
}

impl Command for UpdateCategoryOrderingCommand {}

#[derive(Debug, Clone)]
pub enum CategoriesCommand {
    Add {
        id: CategoryId
    },
    Remove {
        id: CategoryId
    },
    Update {
        new: BTreeMap<i32, Uuid>
    }
}

impl Command for CategoriesCommand {}
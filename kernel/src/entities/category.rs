mod id;
mod name;

pub use self::{
    id::*,
    name::*,
};

use std::collections::BTreeMap;

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

use crate::entities::product::ProductId;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    products: BTreeMap<i32, ProductId>
}

impl Category {
    pub fn new(
        id: CategoryId,
        name: CategoryName,
    ) -> Category {
        Category {
            id,
            name,
            products: BTreeMap::new()
        }
    }
    
    pub fn id(&self) -> &CategoryId {
        &self.id
    }
    
    pub fn name(&self) -> &CategoryName {
        &self.name
    }
    
    pub fn products(&self) -> &BTreeMap<i32, ProductId> {
        &self.products
    }
}
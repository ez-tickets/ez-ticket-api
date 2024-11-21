mod id;
mod name;
mod ordering;
mod ordering_product;

pub use self::id::*;
pub use self::name::*;
pub use self::ordering::*;
pub use self::ordering_product::*;

use crate::commands::CategoryCommand;
use crate::events::CategoryEvent;
use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use nitinol::agent::{Context, Publisher};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    products: BTreeSet<OrderingProduct>,
}

impl Category {
    pub fn new(
        id: CategoryId,
        name: CategoryName,
        products: BTreeSet<OrderingProduct>,
    ) -> Self {
        Self {
            id,
            name,
            products,
        }
    }

    pub fn create(id: CategoryId, name: CategoryName) -> Self {
        Self {
            id,
            name,
            products: BTreeSet::new(),
        }
    }
}

impl Category {
    pub fn id(&self) -> &CategoryId {
        &self.id
    }

    pub fn name(&self) -> &CategoryName {
        &self.name
    }
    
    pub fn products(&self) -> &BTreeSet<OrderingProduct> {
        &self.products
    }
}


#[async_trait]
impl Publisher<CategoryCommand> for Category {
    type Event = CategoryEvent;
    type Rejection = ();

    async fn publish(&self, command: CategoryCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        todo!()
    }
}
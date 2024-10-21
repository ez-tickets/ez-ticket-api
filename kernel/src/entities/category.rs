mod id;
mod name;
mod ordering;
mod ordering_product;

pub use self::id::*;
pub use self::name::*;
pub use self::ordering::*;
pub use self::ordering_product::*;

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    ordering: CategoryOrdering,
    products: BTreeSet<OrderingProduct>,
}

impl Category {
    pub fn new(
        id: CategoryId,
        name: CategoryName,
        ordering: CategoryOrdering,
        products: BTreeSet<OrderingProduct>,
    ) -> Self {
        Self {
            id,
            name,
            ordering,
            products,
        }
    }

    pub fn create(id: CategoryId, name: CategoryName, ordering: CategoryOrdering) -> Self {
        Self {
            id,
            name,
            ordering,
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

    pub fn ordering(&self) -> &CategoryOrdering {
        &self.ordering
    }

    pub fn products(&self) -> &BTreeSet<OrderingProduct> {
        &self.products
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl PartialOrd<Self> for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Category {}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ordering == other.ordering
    }
}

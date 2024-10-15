mod id;
mod name;
mod ordering;

pub use self::name::*;
pub use self::id::*;
pub use self::ordering::*;
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    ordering: CategoryOrdering,
}

impl Category {
    pub fn new(id: CategoryId, name: CategoryName, ordering: CategoryOrdering) -> Self {
        Self { id, name, ordering }
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

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::entities::category::CategoryId;

/// This event that is produced when a [`CategoriesCommand`](crate::process::commands::categories::CategoriesCommand) 
/// is applied to a [`Categories`](crate::entities::categories::Categories) entity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoriesEvent {
    AddedCategory {
        id: CategoryId,
    },
    RemovedCategory {
        id: CategoryId,
    },
    ChangedOrdering {
        new: BTreeMap<i32, CategoryId>
    }
}


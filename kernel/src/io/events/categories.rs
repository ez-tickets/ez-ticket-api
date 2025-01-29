use crate::entities::category::CategoryId;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// This event that is produced when a [`CategoriesCommand`](crate::io::commands::categories::CategoriesCommand)
/// is applied to a [`Categories`](crate::entities::categories::Categories) entity.
#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "serde_json::to_vec", dec = "serde_json::from_slice")]
pub enum CategoriesEvent {
    AddedCategory { id: CategoryId, ordering: i64 },
    RemovedCategory { new: BTreeMap<i64, CategoryId> },
    ChangedOrdering { new: BTreeMap<i64, CategoryId> },
}

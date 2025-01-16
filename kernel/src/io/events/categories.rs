use crate::entities::category::CategoryId;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// This event that is produced when a [`CategoriesCommand`](crate::io::commands::categories::CategoriesCommand)
/// is applied to a [`Categories`](crate::entities::categories::Categories) entity.
#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "flexbuffers::to_vec", dec = "flexbuffers::from_slice")]
pub enum CategoriesEvent {
    AddedCategory { id: CategoryId },
    RemovedCategory { id: CategoryId },
    ChangedOrdering { new: BTreeMap<i32, CategoryId> },
}

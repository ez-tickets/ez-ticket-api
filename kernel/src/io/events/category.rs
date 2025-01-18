use crate::entities::category::{CategoryId, CategoryName};
use crate::entities::product::ProductId;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "serde_json::to_vec", dec = "serde_json::from_slice")]
pub enum CategoryEvent {
    Created { id: CategoryId, name: CategoryName },
    Renamed { new: CategoryName },
    Deleted { id: CategoryId },

    AddedProduct { id: ProductId },
    RemovedProduct { id: ProductId },
    ChangedProductOrdering { new: BTreeMap<i64, ProductId> },
}

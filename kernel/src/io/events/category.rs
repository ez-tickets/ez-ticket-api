use crate::entities::category::{CategoryId, CategoryName};
use crate::entities::product::ProductId;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "flexbuffers::to_vec", dec = "flexbuffers::from_slice")]
pub enum CategoryEvent {
    Created { id: CategoryId, name: CategoryName },
    Renamed { new: CategoryName },
    Deleted,

    AddedProduct { id: ProductId },
    RemovedProduct { id: ProductId },
    ChangedProductOrdering { new: BTreeMap<i32, ProductId> },
}

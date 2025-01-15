use std::collections::BTreeMap;
use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use crate::entities::category::{CategoryId, CategoryName};
use crate::entities::product::ProductId;

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "flexbuffers::to_vec", dec = "flexbuffers::from_slice")]
pub enum CategoryEvent { 
    Created {
        id: CategoryId,
        name: CategoryName
    },
    Renamed {
        new: CategoryName
    },
    Deleted,
    
    AddedProduct {
        id: ProductId
    },
    RemovedProduct {
        id: ProductId
    },
    ChangedProductOrdering {
        new: BTreeMap<i32, ProductId>
    }
}
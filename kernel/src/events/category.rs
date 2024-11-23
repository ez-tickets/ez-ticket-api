use crate::entities::{CategoryId, CategoryName, ProductId};
use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryEvent {
    Created {
        id: CategoryId,
        name: CategoryName,
    },
    UpdatedName {
        name: CategoryName,
    },
    Deleted,
    AddedProduct {
        product_id: ProductId,
    },
    UpdatedProductOrdering {
        ordering: BTreeMap<i32, ProductId>,
    },
    RemovedProduct {
        product_id: ProductId
    }
}

impl Event for CategoryEvent {
    const REGISTRY_KEY: &'static str = "category-event";

    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bytes)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoriesEvent {
    Added { id: CategoryId },
    Removed { id: CategoryId },
    Updated { new: BTreeMap<i32, CategoryId> }
}

impl Event for CategoriesEvent {
    const REGISTRY_KEY: &'static str = "categories-event";

    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bytes)?)
    }
}

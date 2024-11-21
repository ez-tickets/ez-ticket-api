use std::collections::BTreeMap;
use crate::entities::{CategoryId, CategoryName, CategoryOrdering, OrderingProduct};
use serde::{Deserialize, Serialize};
use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryEvent {
    Created {
        name: CategoryName,
        ordering: CategoryOrdering,
    },
    UpdatedName {
        name: CategoryName,
    },
    Deleted,
    AddedProduct {
        product: OrderingProduct,
    },
    UpdatedProductOrdering {
        ordering_product: OrderingProduct,
    },
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

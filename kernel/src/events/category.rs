use std::collections::BTreeMap;

use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;
use serde::{Deserialize, Serialize};

use crate::entities::{CategoryId, CategoryName, CatalogId};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryEvent {
    Created {
        id: CategoryId,
        name: CategoryName,
    },
    UpdatedName {
        id: CategoryId,
        name: CategoryName,
    },
    Deleted {
        id: CategoryId,
    },
    AddedCatalog {
        id: CategoryId,
        ordering: i32,
        catalog: CatalogId,
    },
    UpdatedCatalogOrdering {
        id: CategoryId,
        ordering: BTreeMap<i32, CatalogId>,
    },
    RemovedCatalog {
        id: CategoryId,
        catalog: CatalogId
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
    Added { id: CategoryId, ordering: i32 },
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

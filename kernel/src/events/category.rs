use crate::entities::{CategoryName, CategoryOrdering, OrderingProduct};
use serde::{Deserialize, Serialize};
use spectroscopy::errors::{DeserializeError, SerializeError};
use spectroscopy::Event;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryEvent {
    Created {
        name: CategoryName,
        ordering: CategoryOrdering,
    },
    UpdatedName {
        name: CategoryName,
    },
    UpdatedOrdering {
        ordering: CategoryOrdering,
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

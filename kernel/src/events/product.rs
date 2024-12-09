use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;
use serde::{Deserialize, Serialize};

use crate::entities::{ProductId, ProductName};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductEvent {
    Created {
        id: ProductId,
        name: ProductName,
    },
    UpdatedName {
        id: ProductId,
        name: ProductName,
    },
    Deleted {
        id: ProductId,
    },
}

impl Event for ProductEvent {
    const REGISTRY_KEY: &'static str = "product-event";

    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bytes)?)
    }
}
use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;
use serde::{Deserialize, Serialize};

use crate::entities::{CatalogDesc, CatalogId, CatalogName, MainProduct, Price};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CatalogEvent {
    Created {
        id: CatalogId,
        name: CatalogName,
        desc: CatalogDesc,
        price: Price,
        main: MainProduct,
    },
    UpdatedName {
        id: CatalogId,
        name: CatalogName,
    },
    UpdatedDesc {
        id: CatalogId,
        desc: CatalogDesc,
    },
    UpdatedPrice {
        id: CatalogId,
        price: Price,
    },
    Deleted,
}

impl Event for CatalogEvent {
    const REGISTRY_KEY: &'static str = "catalog-event";

    fn as_bytes(&self) -> Result<Vec<u8>, SerializeError> {
        Ok(flexbuffers::to_vec(self)?)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError> {
        Ok(flexbuffers::from_slice(bytes)?)
    }
}
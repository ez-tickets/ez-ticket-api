use nitinol::errors::{DeserializeError, SerializeError};
use nitinol::Event;
use serde::{Deserialize, Serialize};
use crate::entities::{Price, ProductDescription, ProductName, Stock};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProductEvent {
    Registered {
        name: ProductName,
        desc: ProductDescription,
        price: Price,
        stock: Stock,
    },
    UpdatedName {
        name: ProductName,
    },
    UpdatedDescription {
        desc: ProductDescription,
    },
    StockedIn {
        amount: i32,
    },
    StockedOut {
        amount: i32,
    },
    UpdatedPrice {
        price: Price,
    },
    Deleted,
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
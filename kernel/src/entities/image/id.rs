use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;
use crate::entities::product::ProductId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ImageId(Uuid);

impl ImageId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<ProductId> for ImageId {
    fn from(value: ProductId) -> Self {
        Self(value.into())
    }
}
 
impl AsRef<Uuid> for ImageId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<ImageId> for Uuid {
    fn from(id: ImageId) -> Self {
        id.0
    }
}

impl Display for ImageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

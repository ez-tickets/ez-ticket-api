use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::entities::{CatalogId, ProductId};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ImageId(Uuid);

impl ImageId {
    pub fn new(id: impl Into<Uuid>) -> ImageId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for ImageId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<ImageId> for Uuid {
    fn from(content: ImageId) -> Uuid {
        content.0
    }
}

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image({})", self.0)
    }
}

impl From<ProductId> for ImageId {
    fn from(id: ProductId) -> Self {
        Self(id.into())
    }
}

impl From<CatalogId> for ImageId {
    fn from(id: CatalogId) -> Self {
        Self(id.into())
    }
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ImageId(Uuid);

impl ImageId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
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

impl Default for ImageId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
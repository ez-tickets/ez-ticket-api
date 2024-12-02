mod id;
mod bind_id;

pub use self::bind_id::*;
pub use self::id::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Image {
    bind: BindId,
    resource: ImageId
}

impl Image {
    pub fn new(bindable: impl Into<BindId>, resource: ImageId) -> Self {
        Self { bind: bindable.into(), resource }
    }
}

impl Image {
    pub fn bind(&self) -> BindId {
        self.bind
    }
    
    pub fn resource(&self) -> ImageId {
        self.resource
    }
}

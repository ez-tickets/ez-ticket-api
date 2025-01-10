mod id;

pub use self::id::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    id: ImageId,
    image: Vec<u8>,
}

impl Image {
    pub fn new(id: ImageId, image: Vec<u8>) -> Image {
        Image { id, image }
    }

    pub fn id(&self) -> &ImageId {
        &self.id
    }

    pub fn image(&self) -> &Vec<u8> {
        &self.image
    }
}

mod id;

use std::fmt::{Debug, Formatter};
pub use self::id::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
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

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("id", &self.id)
            .field("image", &format!("{} bytes", &self.image.len()))
            .finish()
    }
}
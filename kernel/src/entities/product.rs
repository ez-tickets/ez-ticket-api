mod id;
mod name;
mod price;
mod desc;

pub use self::{
    id::*,
    name::*,
    desc::*,
    price::*,
};

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

use crate::entities::image::ImageId;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    desc: ProductDesc,
    price: ProductPrice,
    image: ImageId
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        desc: ProductDesc,
        price: ProductPrice,
        image: ImageId
    ) -> Product {
        Product {
            id,
            name,
            desc,
            price,
            image
        }
    }

    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn desc(&self) -> &ProductDesc {
        &self.desc
    }

    pub fn price(&self) -> &ProductPrice {
        &self.price
    }
    
    pub fn image(&self) -> &ImageId {
        &self.image
    }
}
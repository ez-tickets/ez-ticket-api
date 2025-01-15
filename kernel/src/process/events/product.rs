use nitinol::macros::Event;
use serde::{Deserialize, Serialize};
use crate::entities::product::{ProductDesc, ProductId, ProductName, ProductPrice};
use crate::entities::image::ImageId;

#[derive(Debug, Clone, Event, Deserialize, Serialize)]
#[persist(enc = "flexbuffers::to_vec", dec = "flexbuffers::from_slice")]
pub enum ProductEvent {
    Registered {
        id: ProductId,
        name: ProductName,
        desc: ProductDesc,
        price: ProductPrice,
        image: ImageId
    },
    RenamedProductName {
        new: ProductName
    },
    EditedProductDesc {
        new: ProductDesc
    },
    ChangedProductPrice {
        new: ProductPrice
    },
    Deleted
}

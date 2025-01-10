use serde::{Deserialize, Serialize};
use crate::entities::product::{ProductDesc, ProductId, ProductName, ProductPrice};
use crate::entities::image::ImageId;

#[derive(Debug, Clone, Deserialize, Serialize)]
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

use crate::entities::image::ImageId;
use crate::entities::product::{ProductDesc, ProductName, ProductPrice};
use nitinol::macros::Command;

/// This command is used to interact with a [`Product`](crate::entities::product::Product) entity.
/// 
/// # Commands
/// | Command             | Description                      |
/// |---------------------|----------------------------------|
/// | `Register`          | Registers a new product.         |
/// | `RenameProductName` | Renames the product.             |
/// | `EditProductDesc`   | Edits the product description.   |
/// | `ChangeProductPrice`| Changes the product price.       |
/// | `Delete`            | Deletes the product.             |
#[derive(Debug, Clone, Command)]
pub enum ProductCommand {
    Register { 
        name: ProductName, 
        desc: ProductDesc, 
        price: ProductPrice,
        image: ImageId
    },
    RenameProductName {
        new: ProductName
    },
    EditProductDesc {
        new: ProductDesc
    },
    ChangeProductPrice {
        new: ProductPrice
    },
    Delete,
}


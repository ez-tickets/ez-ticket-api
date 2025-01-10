use uuid::Uuid;


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
#[derive(Debug, Clone)]
pub enum ProductCommand {
    Register { 
        name: String, 
        desc: String, 
        price: i64,
        image: Uuid
    },
    RenameProductName {
        new: String
    },
    EditProductDesc {
        new: String
    },
    ChangeProductPrice {
        new: i64
    },
    Delete,
}


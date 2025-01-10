use std::collections::BTreeMap;
use crate::entities::product::ProductId;

/// CategoryCommand is a command that can be applied to a [`Category`](crate::entities::category::Category) entity.
/// 
/// # Commands
/// - `Create`: Creates a new category.
/// - `Rename`: Renames the category.
/// - `Delete`: Deletes the category.
/// - `ChangeProductOrdering`: Changes the ordering of the products.
///   - **Cannot be added or deleted within this command**.
#[derive(Debug, Clone)]
pub enum CategoryCommand {
    Create {
        name: String
    },
    Rename {
        new: String
    },
    Delete,
    
    AddProduct {
        id: ProductId
    },
    RemoveProduct {
        id: ProductId
    },
    ChangeProductOrdering {
        new: BTreeMap<i32, ProductId>
    }
}

use crate::entities::category::CategoryName;
use crate::entities::product::ProductId;
use nitinol::macros::Command;
use std::collections::BTreeMap;

/// CategoryCommand is a command that can be applied to a [`Category`](crate::entities::category::Category) entity.
///
/// # Commands
/// - `Create`: Creates a new category.
/// - `Rename`: Renames the category.
/// - `Delete`: Deletes the category.
/// - `AddProduct`: Adds a product to the category.
/// - `RemoveProduct`: Removes a product from the category.
/// - `ChangeProductOrdering`: Changes the ordering of the products.
///   - **Cannot be added or deleted within this command**.
#[derive(Debug, Clone, Command)]
pub enum CategoryCommand {
    Create { name: CategoryName },
    Rename { new: CategoryName },
    Delete,

    AddProduct { id: ProductId },
    RemoveProduct { id: ProductId },
    ChangeProductOrdering { new: BTreeMap<i64, ProductId> },
}

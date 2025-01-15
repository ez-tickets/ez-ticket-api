use std::collections::BTreeMap;
use nitinol::macros::Command;
use crate::entities::category::CategoryId;

/// CategoriesCommand is a command that can be applied to a [`Categories`](crate::entities::categories::Categories) entity.
/// 
/// # Commands
/// - `AddCategory`: Adds a category to the list of categories.
/// - `RemoveCategory`: Removes a category from the list of categories.
/// - `ChangeOrdering`: Changes the ordering of the categories.
///   - **Cannot be added or deleted within this command**.
/// 
#[derive(Debug, Clone, Command)]
pub enum CategoriesCommand {
    AddCategory {
        id: CategoryId,
    },
    RemoveCategory {
        id: CategoryId,
    },
    ChangeOrdering {
        new: BTreeMap<i32, CategoryId>
    }
}

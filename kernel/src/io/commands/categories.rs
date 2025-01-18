use std::collections::BTreeMap;
use error_stack::Report;
use nitinol::macros::Command;

use crate::entities::category::CategoryId;
use crate::errors::FormationError;
use crate::io::events::CategoryEvent;

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
    AddCategory { id: CategoryId },
    RemoveCategory { id: CategoryId },
    ChangeOrdering { new: BTreeMap<i64, CategoryId> },
}

impl TryFrom<CategoryEvent> for CategoriesCommand {
    type Error = Report<FormationError>;
    fn try_from(value: CategoryEvent) -> Result<Self, Self::Error> {
        match value {
            CategoryEvent::Created { id, .. } => Ok(Self::AddCategory { id }),
            CategoryEvent::Deleted { id } => Ok(Self::RemoveCategory { id }),
            _ => Err(Report::new(FormationError).attach_printable("Cannot convert event to command."))?,
        }
    }
}
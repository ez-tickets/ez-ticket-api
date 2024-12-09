use crate::entities::{CategoryId, CategoryName, CatalogId};
use nitinol::Command;
use std::collections::BTreeMap;
use error_stack::Report;
use crate::errors::KernelError;
use crate::events::CategoryEvent;

#[derive(Debug, Clone)]
pub enum CategoryCommand {
    Create { name: CategoryName },
    UpdateName { name: String },
    Delete,
    AddCatalog { catalog: CatalogId },
    UpdateCatalogOrdering { ordering: BTreeMap<i32, CatalogId> },
    RemoveCatalog { catalog: CatalogId }
}


impl Command for CategoryCommand {}

#[derive(Debug, Clone)]
pub enum CategoriesCommand {
    Add {
        id: CategoryId
    },
    Remove {
        id: CategoryId
    },
    Update {
        new: BTreeMap<i32, CategoryId>
    }
}

impl Command for CategoriesCommand {}

impl TryFrom<CategoryEvent> for CategoriesCommand {
    type Error = Report<KernelError>;

    fn try_from(value: CategoryEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            CategoryEvent::Created { id, .. } => Self::Add { id },
            CategoryEvent::Deleted { id } => Self::Remove { id },
            _ => return Err(Report::new(KernelError::Invalid)
                .attach_printable("Conversion from CategoryCommand to CategoryCommand is supported only between Create/Add and Delete/Remove."))
        })
    }
}

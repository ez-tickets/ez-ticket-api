use error_stack::Report;
use serde::Deserialize;
use kernel::commands::CategoryCommand;
use kernel::entities::{CategoryId, CategoryName};
use kernel::errors::KernelError;

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    name: String
}

impl TryFrom<CreateCategory> for CategoryCommand {
    type Error = Report<KernelError>;

    fn try_from(value: CreateCategory) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::Create {
            name: CategoryName::new(value.name)
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct FindCategory {
    pub id: CategoryId
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryName {
    name: String
}

impl TryFrom<UpdateCategoryName> for CategoryCommand {
    type Error = Report<KernelError>;

    fn try_from(value: UpdateCategoryName) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::UpdateName {
            name: value.name,
        })
    }
}

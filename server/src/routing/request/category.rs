use error_stack::Report;
use serde::Deserialize;
use kernel::commands::CategoryCommand;
use kernel::entities::CategoryName;
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
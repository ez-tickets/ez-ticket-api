use std::collections::BTreeMap;
use error_stack::{Report, ResultExt};
use serde::Deserialize;
use kernel::entities::category::{CategoryId, CategoryName};
use kernel::entities::product::ProductId;
use kernel::io::commands::{CategoriesCommand, CategoryCommand};
use crate::errors::ServerError;

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    name: String,
}

impl TryFrom<CreateCategory> for CategoryCommand {
    type Error = Report<ServerError>;

    fn try_from(value: CreateCategory) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::Create {
            name: CategoryName::new(value.name).change_context_lazy(|| ServerError::Validation)?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct RenameCategory {
    name: String,
}

impl TryFrom<RenameCategory> for CategoryCommand {
    type Error = Report<ServerError>;

    fn try_from(value: RenameCategory) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::Rename {
            new: CategoryName::new(value.name).change_context_lazy(|| ServerError::Validation)?,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ChangeCategoryOrdering(BTreeMap<i64, CategoryId>);

impl TryFrom<ChangeCategoryOrdering> for CategoriesCommand {
    type Error = Report<ServerError>;

    fn try_from(value: ChangeCategoryOrdering) -> Result<Self, Self::Error> {
        Ok(CategoriesCommand::ChangeOrdering {
            new: value.0,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct AddProduct {
    pub product: ProductId,
}

impl TryFrom<AddProduct> for CategoryCommand {
    type Error = Report<ServerError>;

    fn try_from(value: AddProduct) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::AddProduct {
            id: value.product,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct ChangeProductOrdering(BTreeMap<i64, ProductId>);

impl TryFrom<ChangeProductOrdering> for CategoryCommand {
    type Error = Report<ServerError>;

    fn try_from(value: ChangeProductOrdering) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::ChangeProductOrdering {
            new: value.0,
        })
    }
}
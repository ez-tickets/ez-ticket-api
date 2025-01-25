use crate::errors::ServerError;
use error_stack::{Report, ResultExt};
use kernel::entities::category::{CategoryId, CategoryName};
use kernel::entities::product::ProductId;
use kernel::io::commands::{CategoriesCommand, CategoryCommand};
use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
struct OrderedCategory {
    ordering: i64,
    #[cfg_attr(feature = "apidoc", schema(value_type = Uuid))]
    id: CategoryId,
}

impl PartialEq for OrderedCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OrderedCategory {}

impl PartialOrd for OrderedCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
pub struct ChangeCategoryOrdering(BTreeSet<OrderedCategory>);

impl TryFrom<ChangeCategoryOrdering> for CategoriesCommand {
    type Error = Report<ServerError>;

    fn try_from(value: ChangeCategoryOrdering) -> Result<Self, Self::Error> {
        Ok(CategoriesCommand::ChangeOrdering {
            new: value.0.into_iter()
                .map(|category| (category.ordering, category.id))
                .collect(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
pub struct AddProduct {
    #[cfg_attr(feature = "apidoc", schema(value_type = Uuid))]
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
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
struct OrderedProduct {
    ordering: i64,
    #[cfg_attr(feature = "apidoc", schema(value_type = Uuid))]
    id: ProductId,
}

impl PartialEq for OrderedProduct {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OrderedProduct {}

impl PartialOrd for OrderedProduct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedProduct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "apidoc", derive(utoipa::ToSchema))]
pub struct ChangeProductOrdering(BTreeSet<OrderedProduct>);

impl TryFrom<ChangeProductOrdering> for CategoryCommand {
    type Error = Report<ServerError>;

    fn try_from(value: ChangeProductOrdering) -> Result<Self, Self::Error> {
        Ok(CategoryCommand::ChangeProductOrdering {
            new: value.0.into_iter()
                .map(|product| (product.ordering, product.id))
                .collect(),
        })
    }
}
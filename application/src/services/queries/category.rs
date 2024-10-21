use crate::errors::ApplicationError;
use error_stack::Report;
use kernel::entities::{Category, CategoryId};
use std::collections::BTreeSet;

pub trait DependOnCategoryQueryService: 'static + Sync + Send {
    type CategoryQueryService: CategoryQueryService;
    fn category_query_service(&self) -> &Self::CategoryQueryService;
}

#[async_trait::async_trait]
pub trait CategoryQueryService: 'static + Sync + Send {
    async fn find_all_category(&self) -> Result<BTreeSet<CategoryId>, Report<ApplicationError>>;
    async fn find_all_product_by_category_id(
        &self,
        id: &CategoryId,
    ) -> Result<BTreeSet<Category>, Report<ApplicationError>>;
}

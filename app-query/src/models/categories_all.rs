use crate::errors::QueryError;
use crate::models::OrderedCategory;
use async_trait::async_trait;
use error_stack::Report;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct AllCategories(pub BTreeSet<OrderedCategory>);

pub trait DependOnGetAllCategoriesQueryService: 'static + Sync + Send {
    type GetAllCategoriesQueryService: GetAllCategoriesQueryService;
    fn get_all_categories_query_service(&self) -> &Self::GetAllCategoriesQueryService;
}

#[async_trait]
pub trait GetAllCategoriesQueryService: 'static + Sync + Send {
    async fn get_all_categories(&self) -> Result<AllCategories, Report<QueryError>>;
}

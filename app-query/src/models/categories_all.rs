use std::collections::BTreeMap;
use async_trait::async_trait;
use error_stack::Report;
use serde::Serialize;
use crate::errors::QueryError;
use crate::models::category::Category;

#[derive(Serialize, sqlx::FromRow)]
pub struct AllCategories(pub BTreeMap<i64, Category>);

pub trait DependOnGetAllCategoriesQueryService: 'static + Sync + Send {
    type GetAllCategoriesQueryService: GetAllCategoriesQueryService;
    fn get_all_categories_query_service(&self) -> &Self::GetAllCategoriesQueryService;
}

#[async_trait]
pub trait GetAllCategoriesQueryService: 'static + Sync + Send {
    async fn get_all_categories(&self) -> Result<AllCategories, Report<QueryError>>;
}

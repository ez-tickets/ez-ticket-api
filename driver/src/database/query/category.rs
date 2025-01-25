use crate::errors::FailedQuery;
use app_query::errors::QueryError;
use app_query::models::{AllCategories, GetAllCategoriesQueryService, OrderedCategory};
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use sqlx::SqliteConnection;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct CategoryQueryService {
    pool: sqlx::SqlitePool
}

impl CategoryQueryService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GetAllCategoriesQueryService for CategoryQueryService {
    async fn get_all_categories(&self) -> Result<AllCategories, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError::Driver)?;
        let all = InternalCategoryQueryService::get_all_categories(&mut con).await
            .change_context_lazy(|| QueryError::Driver)?;
        Ok(all)
    }
}

pub(crate) struct InternalCategoryQueryService;

impl InternalCategoryQueryService {
    pub async fn get_all_categories(con: &mut SqliteConnection) -> Result<AllCategories, Report<FailedQuery>> {
        // language=sqlite
        let all = sqlx::query_as::<_, OrderedCategory>(r#"
            SELECT 
                co.ordering, 
                c.id, 
                c.name 
            FROM 
                categories c
            JOIN 
                categories_ordering co ON c.id = co.category
        "#)
            .fetch_all(&mut *con)
            .await
            .change_context_lazy(|| FailedQuery)?;
        
        let categories = all.into_iter().collect::<BTreeSet<OrderedCategory>>();
        
        Ok(AllCategories(categories))
    }
}
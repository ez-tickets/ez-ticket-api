use std::cmp::Ordering;
use std::collections::BTreeSet;
use async_trait::async_trait;
use error_stack::Report;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::QueryError;

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub ordering: i32,
}

impl Eq for Category {}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        Some(self.cmp(other)) 
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

#[async_trait]
pub trait CategoryQueryService: 'static + Sync + Send {
    async fn find_all_category(&self) -> Result<BTreeSet<Category>, Report<QueryError>>;
}

pub trait DependOnCategoryQueryService: 'static + Sync + Send {
    type CategoryQueryService: CategoryQueryService;
    fn category_query_service(&self) -> &Self::CategoryQueryService;
}
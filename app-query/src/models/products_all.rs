use error_stack::Report;
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
use uuid::Uuid;

use crate::errors::QueryError;
use crate::models::Product;

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct AllProduct(pub HashSet<Product>);

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct OrderedProduct {
    pub ordering: i64,
    pub id: Uuid,
    pub name: String,
    pub price: i64,
}

impl Eq for OrderedProduct {}

impl PartialEq<Self> for OrderedProduct {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) 
            || self.ordering.eq(&other.ordering)
    }
}

impl PartialOrd<Self> for OrderedProduct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedProduct {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.cmp(&other.ordering)
            .then_with(|| self.id.cmp(&other.id))
    }
}


#[derive(Serialize, utoipa::ToSchema)]
pub struct OrderedProducts(pub BTreeSet<OrderedProduct>);


pub trait DependOnGetAllProductQueryService: 'static + Sync + Send {
    type GetAllProductQueryService: GetAllProductQueryService;
    fn get_all_product_query_service(&self) -> &Self::GetAllProductQueryService;
}

#[async_trait::async_trait]
pub trait GetAllProductQueryService: 'static + Sync + Send {
    async fn get_all_product(&self) -> Result<AllProduct, Report<QueryError>>;
    async fn get_all_product_by_category(&self, category: &Uuid) -> Result<OrderedProducts, Report<QueryError>>;
}
use std::collections::{BTreeMap, HashSet};
use error_stack::Report;
use serde::Serialize;
use uuid::Uuid;
use crate::errors::QueryError;
use crate::models::Product;

#[derive(Serialize, sqlx::FromRow)]
pub struct AllProduct(pub HashSet<Product>);

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderedProduct {
    pub ordering: i64,
    pub id: Uuid,
    pub name: String,
    pub price: i64,
}

#[derive(Serialize)]
pub struct OrderedProducts(pub BTreeMap<i64, Product>);


pub trait DependOnGetAllProductQueryService: 'static + Sync + Send {
    type GetAllProductQueryService: GetAllProductQueryService;
    fn get_all_product_query_service(&self) -> &Self::GetAllProductQueryService;
}

#[async_trait::async_trait]
pub trait GetAllProductQueryService: 'static + Sync + Send {
    async fn get_all_product(&self) -> Result<AllProduct, Report<QueryError>>;
    async fn get_all_product_by_category(&self, category: &Uuid) -> Result<OrderedProducts, Report<QueryError>>;
}
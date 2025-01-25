use std::hash::{Hash, Hasher};
use async_trait::async_trait;
use error_stack::Report;
use serde::Serialize;
use uuid::Uuid;
use crate::errors::QueryError;

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: i64,
}

impl PartialEq<Self> for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Product {}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.price.hash(state);
    }
}


#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct ProductDetails {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub price: i64,
}


impl PartialEq<Self> for ProductDetails {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ProductDetails {}

impl Hash for ProductDetails {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.desc.hash(state);
        self.price.hash(state);
    }
}

pub trait DependOnGetProductQueryService: 'static + Sync + Send {
    type GetProductQueryService: GetProductQueryService;
    fn get_product_query_service(&self) -> &Self::GetProductQueryService;
}

#[async_trait]
pub trait GetProductQueryService: 'static + Sync + Send {
    async fn get_product_details(&self, product: &Uuid) -> Result<ProductDetails, Report<QueryError>>;
}

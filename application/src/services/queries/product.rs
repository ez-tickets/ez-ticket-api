use std::collections::BTreeSet;
use error_stack::Report;
use kernel::entities::{Product, ProductId};
use crate::errors::ApplicationError;

pub trait DependOnProductQueryService: 'static + Sync + Send {
    type ProductQueryService: ProductQueryService;
    fn product_query_service(&self) -> &Self::ProductQueryService;
}

#[async_trait::async_trait]
pub trait ProductQueryService: 'static + Sync + Send {
    async fn find_all_product(&self) -> Result<BTreeSet<ProductId>, Report<ApplicationError>>;
    async fn find_product_by_id(&self, id: &ProductId) -> Result<Product, Report<ApplicationError>>;
}
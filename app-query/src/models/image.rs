use async_trait::async_trait;
use error_stack::Report;
use uuid::Uuid;
use crate::errors::QueryError;

pub trait DependOnGetProductImageQueryService {
    type GetProductImageQueryService: GetProductImageQueryService;
    fn get_product_image_query_service(&self) -> &Self::GetProductImageQueryService;
}

#[async_trait]
pub trait GetProductImageQueryService: 'static + Sync + Send {
    async fn get_product_image(&self, id: &Uuid) -> Result<Vec<u8>, Report<QueryError>>;
}

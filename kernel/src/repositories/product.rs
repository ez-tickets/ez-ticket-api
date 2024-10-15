use error_stack::Report;
use crate::entities::{Product, ProductId};
use crate::errors::KernelError;

pub trait DependOnProductRepository: 'static + Sync + Send {
    type ProductRepository: ProductRepository;
    fn product_repository(&self) -> &Self::ProductRepository;
}

#[async_trait::async_trait]
pub trait ProductRepository: 'static + Sync + Send {
    async fn create(&self, product: &Product) -> Result<(), Report<KernelError>>;
    async fn update(&self, id: &ProductId, product: &Product) -> Result<(), Report<KernelError>>;
    async fn delete(&self, id: &ProductId) -> Result<(), Report<KernelError>>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Product, Report<KernelError>>;
}
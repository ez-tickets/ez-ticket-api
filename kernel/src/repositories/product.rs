use crate::entities::{Product, ProductId, ProductOption, ProductOptionId};
use crate::errors::KernelError;
use error_stack::Report;

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

pub trait DependOnProductOptionRepository: 'static + Sync + Send {
    type ProductOptionRepository: ProductOptionRepository;
    fn product_option_repository(&self) -> &Self::ProductOptionRepository;
}

#[async_trait::async_trait]
pub trait ProductOptionRepository: 'static + Sync + Send {
    async fn create(
        &self,
        id: &ProductId,
        option: &ProductOption,
    ) -> Result<(), Report<KernelError>>;
    async fn update(
        &self,
        product: &ProductId,
        id: &ProductOptionId,
        option: &ProductOption,
    ) -> Result<(), Report<KernelError>>;
    async fn delete(
        &self,
        product: &ProductId,
        id: &ProductOptionId,
    ) -> Result<(), Report<KernelError>>;
    async fn find_by_id(
        &self,
        product: &ProductId,
        id: &ProductOptionId,
    ) -> Result<ProductOption, Report<KernelError>>;
}

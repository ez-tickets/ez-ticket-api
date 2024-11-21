use crate::entities::{Category, Category};
use crate::errors::KernelError;
use error_stack::Report;

#[async_trait::async_trait]
pub trait CategoryRepository: 'static + Sync + Send {
    async fn create(&self, category: &Category) -> Result<(), Report<KernelError>>;
    async fn update(&self, id: &Category, category: &Category)
                    -> Result<(), Report<KernelError>>;
    async fn delete(&self, id: &Category) -> Result<(), Report<KernelError>>;
    async fn find_by_id(&self, id: &Category) -> Result<Category, Report<KernelError>>;
}

pub trait DependOnCategoryRepository: 'static + Sync + Send {
    type CategoryRepository: CategoryRepository;
    fn category_repository(&self) -> &Self::CategoryRepository;
}

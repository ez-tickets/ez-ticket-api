use crate::adapter::{self, DependOnEventProjector, DependOnProcessManager};
use crate::errors::ApplicationError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::category::{Category, CategoryId};
use kernel::entities::product::{Product, ProductId};
use kernel::io::commands::{CategoryCommand, ProductCommand};

impl<T> RegisterProductWithCategoryWorkflow for T 
where 
    T
    : DependOnProcessManager
    + DependOnEventProjector
{}

pub trait DependOnRegisterProductWithCategoryWorkflow: 'static + Sync + Send {
    type RegisterProductWithCategoryWorkflow: RegisterProductWithCategoryWorkflow;
    fn register_product_with_category_workflow(&self) -> &Self::RegisterProductWithCategoryWorkflow;
}

#[async_trait]
pub trait RegisterProductWithCategoryWorkflow: 'static + Send + Sync 
where
    Self: DependOnProcessManager
        + DependOnEventProjector
{
    async fn execute(&self, category_id: CategoryId, reg: ProductCommand) -> Result<(), Report<ApplicationError>> {
        let manager = self.process_manager();
        
        let ProductCommand::Register { .. } = &reg else {
            return Err(Report::new(ApplicationError::InvalidCommand)
                .attach_printable("Workflow only accepts `ProductCommand::Register { .. }`."));
        };
        
        let category = adapter::utils::find_or_replay::<Category>(category_id, manager, self.event_projector()).await?;
        
        let product_id = ProductId::default();
        let product = Product::try_from((product_id, reg.clone()))
            .change_context_lazy(|| ApplicationError::Formation)?;
        
        let product = manager.spawn(product_id, product, 0).await
            .change_context_lazy(|| ApplicationError::Process)?;
        
        product.employ(reg).await
            .change_context_lazy(|| ApplicationError::Process)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        let cmd = CategoryCommand::AddProduct { id: product_id };
        
        category.employ(cmd).await
            .change_context_lazy(|| ApplicationError::Process)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        Ok(())
    }
}
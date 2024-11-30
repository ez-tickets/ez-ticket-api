use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::process::registry::ProcessSystem;
use kernel::commands::{CategoriesCommand, CategoryCommand};
use kernel::entities::{Category, CategoryId};
use crate::adapter::{DependOnProcessExtension, DependOnProcessRegistry, DependOnEventProjector};
use crate::errors::ApplicationError;
use crate::services::commands::{CategoriesCommandExecutor, DependOnCategoriesCommandExecutor};

impl<T> CategoryCommandExecutor for T
where
    T: DependOnEventProjector
     + DependOnProcessRegistry
     + DependOnProcessExtension
     + DependOnCategoriesCommandExecutor,
{}


pub trait DependOnCategoryCommandExecutor: 'static + Sync + Send {
    type CategoryCommandExecutor: CategoryCommandExecutor;
    fn category_command_executor(&self) -> &Self::CategoryCommandExecutor;
}

#[rustfmt::skip]
#[async_trait]
pub trait CategoryCommandExecutor: 'static + Sync + Send 
where 
    Self: DependOnEventProjector 
        + DependOnProcessRegistry
        + DependOnProcessExtension
        + DependOnCategoriesCommandExecutor
{
    async fn execute(&self, id: Option<CategoryId>, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>> {
        if let CategoryCommand::Create { name  } = &cmd {
            let id = CategoryId::default();
            let new = Category::create(id, name.clone());
            
            let refs = self.process_registry()
                .spawn(id, new, 0, self.process_extension())
                .await
                .change_context_lazy(|| ApplicationError::Other)?;
            
            let event = refs.publish(cmd).await
                .change_context_lazy(|| ApplicationError::Other)?
                .change_context_lazy(|| ApplicationError::Kernel)?;
            
            let categories = CategoriesCommand::try_from(event.clone())
                .change_context_lazy(|| ApplicationError::Kernel)?;
            
            self.categories_command_executor()
                .execute(categories)
                .await?;
            
            refs.apply(event).await
                .change_context_lazy(|| ApplicationError::Other)?;
            
            return Ok(())
        }
        
        let Some(category) = id else {
            return Err(Report::new(ApplicationError::MissingId {
                entity: "Category"
            }))
        };
        
        let refs = match self.process_registry()
            .find::<Category>(category).await
            .change_context_lazy(|| ApplicationError::Other)? 
        {
            Some(refs) => refs,
            None => {
                let (replay, seq) = self.projector()
                    .projection_to_latest::<Category>(category, None)
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?;
                
                self.process_registry()
                    .spawn(category, replay, seq, self.process_extension())
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?
            }
        };
        
        let event = refs.publish(cmd).await
            .change_context_lazy(|| ApplicationError::Other)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        
        if let Ok(categories) = CategoriesCommand::try_from(event.clone()) {
            self.categories_command_executor()
                .execute(categories)
                .await?;
            
        }
        
        refs.apply(event).await
            .change_context_lazy(|| ApplicationError::Other)?;
        
        Ok(())
    }
}
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::process::registry::ProcessSystem;
use kernel::commands::CategoryCommand;
use kernel::entities::{Category, CategoryId};
use crate::adapter::{DependOnEventStore, DependOnProcessRegistry, DependOnProjector};
use crate::errors::ApplicationError;

#[rustfmt::skip]
#[async_trait]
pub trait CategoryCommandExecutor: 'static + Sync + Send 
where 
    Self: 
        DependOnEventStore 
      + DependOnProjector
      + DependOnProcessRegistry
{
    async fn execute(&self, id: Option<CategoryId>, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>> {
        if let CategoryCommand::Create { name  } = &cmd {
            let id = CategoryId::default();
            let new = Category::create(id, name.clone());
            
            let refs = self.process_registry()
                .spawn(id, new, 0)
                .await
                .change_context_lazy(|| ApplicationError::Other)?;
            
            let event = refs.publish(cmd).await
                .change_context_lazy(|| ApplicationError::Other)?
                .change_context_lazy(|| ApplicationError::Kernel)?;
            
            
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
                    .projection_to_latest::<Category>(&category.to_string(), None)
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?;
                
                self.process_registry()
                    .spawn(category, replay, seq)
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?
            }
        };
        
        let event = refs.publish(cmd).await
            .change_context_lazy(|| ApplicationError::Other)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        refs.apply(event).await
            .change_context_lazy(|| ApplicationError::Other)?;
        
        Ok(())
    }
}
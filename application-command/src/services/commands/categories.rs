use crate::adapter::{DependOnEventProjector, DependOnProcessExtension, DependOnProcessRegistry};
use crate::errors::ApplicationError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::commands::CategoriesCommand;
use kernel::entities::Categories;
use nitinol::process::registry::ProcessSystem;

impl<T> CategoriesCommandExecutor for T
where
    T: DependOnEventProjector 
     + DependOnProcessRegistry 
     + DependOnProcessExtension
{}

pub trait DependOnCategoriesCommandExecutor: 'static + Sync + Send {
    type CategoriesCommandExecutor: CategoriesCommandExecutor;
    fn categories_command_executor(&self) -> &Self::CategoriesCommandExecutor;
}

#[async_trait]
pub trait CategoriesCommandExecutor: 'static + Sync + Send 
where
    Self: DependOnEventProjector
        + DependOnProcessRegistry
        + DependOnProcessExtension
{
    async fn execute(&self, cmd: CategoriesCommand) -> Result<(), Report<ApplicationError>> {
        let refs = match self.process_registry()
            .find::<Categories>(Categories::AGGREGATE_ID).await
            .change_context_lazy(|| ApplicationError::Other)?
        {
            Some(refs) => refs,
            None => {
                let (replay, seq) = self.projector()
                    .projection_to_latest::<Categories>(Categories::AGGREGATE_ID, (Categories::default(), 0))
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?;

                self.process_registry()
                    .spawn(Categories::AGGREGATE_ID, replay, seq, self.process_extension())
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?
            }
        };
        
        let ev = refs.publish(cmd).await
            .change_context_lazy(|| ApplicationError::Other)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        refs.apply(ev).await
            .change_context_lazy(|| ApplicationError::Other)?;
        
        Ok(())
    }
}
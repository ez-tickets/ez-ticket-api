use std::error::Error;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::categories::Categories;
use kernel::io::commands::CategoriesCommand;

use crate::adapter::{self, DependOnEventProjector, DependOnProcessManager};
use crate::errors::ApplicationError;

impl<T> CategoriesCommandService for T 
where 
    T
    : DependOnProcessManager
    + DependOnEventProjector {}


pub trait DependOnCategoriesCommandService: 'static + Sync + Send {
    type CategoriesCommandService: CategoriesCommandService;
    fn categories_command_service(&self) -> &Self::CategoriesCommandService;
}


#[async_trait]
pub trait CategoriesCommandService: 'static + Sync + Send 
where
    Self: DependOnProcessManager
        + DependOnEventProjector
{
    async fn execute(&self, cmd: CategoriesCommand) -> Result<(), Report<ApplicationError>> {
        let manager = self.process_manager();
        
        let refs = adapter::utils::find_or_replay::<Categories>(Categories::ID, manager, self.event_projector()).await?;
        
        let event = refs.publish(cmd).await
            .change_context_lazy(|| ApplicationError::Process)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        refs.apply(event).await
            .change_context_lazy(|| ApplicationError::Process)?;
        
        Ok(())
    }
}
use std::error::Error;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::category::{Category, CategoryId};
use kernel::io::commands::CategoryCommand;

use crate::adapter::{self, DependOnProcessManager, DependOnEventProjector};
use crate::errors::ApplicationError;

impl<T> CategoryCommandService for T 
where T
      : DependOnProcessManager 
      + DependOnEventProjector {}

pub trait DependOnCategoryCommandService: 'static + Sync + Send {
    type CategoryCommandService: CategoryCommandService;
    fn category_command_service(&self) -> &Self::CategoryCommandService;
}

#[async_trait]
pub trait CategoryCommandService: 'static + Sync + Send 
where
    Self: DependOnProcessManager
        + DependOnEventProjector
{
    async fn execute<I, R>(&self, id: I, req: R) -> Result<(), Report<ApplicationError>>
        where
            I: Into<Option<CategoryId>> + Sync + Send,
            R: TryInto<CategoryCommand> + Sync + Send,
            R::Error: Error + Sync + Send + 'static
    {

        let cmd = req.try_into()
            .map_err(|e| Report::new(e).change_context(ApplicationError::Formation))?;
        
        let manager = self.process_manager();
        
        let refs = if let CategoryCommand::Create { .. } = &cmd {
            let id = CategoryId::default();
            
            let category = Category::try_from((id, cmd.clone()))
                .change_context_lazy(|| ApplicationError::Formation)?;
            
            manager.spawn(id, category, 0).await
                .change_context_lazy(|| ApplicationError::Process)?
        } else {
            let id = id.into()
                .ok_or(ApplicationError::RequiredId)?;
            adapter::utils::find_or_replay(id, manager, self.event_projector()).await?
        };
        
        let event = refs.publish(cmd).await
            .change_context_lazy(|| ApplicationError::Process)?
            .change_context_lazy(|| ApplicationError::Kernel)?;
        
        refs.apply(event).await
            .change_context_lazy(|| ApplicationError::Process)?;
        
        Ok(())
    }
}
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::category::{Category, CategoryId};
use kernel::io::commands::{CategoriesCommand, CategoryCommand};
use kernel::io::events::CategoryEvent;

use crate::adapter::{self, DependOnEventProjector, DependOnProcessManager};
use crate::errors::ApplicationError;
use crate::services::categories::{CategoriesCommandService, DependOnCategoriesCommandService};

impl<T> CategoryCommandService for T 
where T
      : DependOnProcessManager 
      + DependOnEventProjector
      + DependOnCategoriesCommandService 
{}

pub trait DependOnCategoryCommandService: 'static + Sync + Send {
    type CategoryCommandService: CategoryCommandService;
    fn category_command_service(&self) -> &Self::CategoryCommandService;
}

#[async_trait]
pub trait CategoryCommandService: 'static + Sync + Send 
where
    Self: DependOnProcessManager
        + DependOnEventProjector
        + DependOnCategoriesCommandService
{
    async fn execute<I>(&self, id: I, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>>
        where
            I: Into<Option<CategoryId>> + Sync + Send,
    {
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

        refs.apply(event.clone()).await
            .change_context_lazy(|| ApplicationError::Process)?;
        
        if let CategoryEvent::Created { .. } | CategoryEvent::Deleted { .. } = event {
            let cmd = CategoriesCommand::try_from(event)
                .change_context_lazy(|| ApplicationError::Formation)?;
            
            self.categories_command_service()
                .execute(cmd)
                .await
                .change_context_lazy(|| ApplicationError::Process)?;
        }
        
        Ok(())
    }
}
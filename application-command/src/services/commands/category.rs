use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::process::registry::ProcessSystem;
use kernel::commands::{CategoriesCommand, CategoryCommand};
use kernel::entities::{Category, CategoryId};
use crate::adapter::{DependOnProcessExtension, DependOnProcessRegistry, DependOnEventProjector};
use crate::errors::ApplicationError;
use crate::services::commands::{CategoriesCommandExecutor, DependOnCategoriesCommandExecutor};

impl<T> CreateCategoryService for T
where
    T: DependOnEventProjector
     + DependOnProcessRegistry
     + DependOnProcessExtension
     + DependOnCategoriesCommandExecutor
{}

pub trait DependOnCreateCategoryService: 'static + Sync + Send {
    type CreateCategoryService: CreateCategoryService;
    fn create_category_service(&self) -> &Self::CreateCategoryService;
}

#[async_trait]
pub trait CreateCategoryService: 'static + Sync + Send 
where
    Self: DependOnEventProjector
        + DependOnProcessRegistry
        + DependOnProcessExtension
        + DependOnCategoriesCommandExecutor
{
    async fn create(&self, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>> {
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
        
        Err(Report::new(ApplicationError::InvalidGivenCommand)
            .attach_printable("Invalid command given to create category"))
    }
}



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
    async fn execute(&self, id: CategoryId, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>> {
        if let CategoryCommand::Create { .. } = cmd {
            return Err(Report::new(ApplicationError::InvalidGivenCommand)
                .attach_printable("This service denies `CategoryCommand::Create`."))
        }
        
        let refs = match self.process_registry()
            .find::<Category>(id).await
            .change_context_lazy(|| ApplicationError::Other)? 
        {
            Some(refs) => refs,
            None => {
                let (replay, seq) = self.projector()
                    .projection_to_latest::<Category>(id, None)
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?;
                
                self.process_registry()
                    .spawn(id, replay, seq, self.process_extension())
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
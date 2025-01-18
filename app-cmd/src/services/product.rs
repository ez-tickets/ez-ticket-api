use std::error::Error;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::product::{Product, ProductId};
use kernel::io::commands::ProductCommand;

use crate::adapter::{self, DependOnProcessManager, DependOnEventProjector};
use crate::errors::ApplicationError;


impl<T> ProductCommandService for T 
where 
    T
    : DependOnProcessManager
    + DependOnEventProjector {}


pub trait DependOnProductCommandService: 'static + Sync + Send {
    type ProductCommandService: ProductCommandService;
    fn product_command_service(&self) -> &Self::ProductCommandService;
}

#[async_trait]
pub trait ProductCommandService: 'static + Sync + Send
where
    Self: DependOnProcessManager
        + DependOnEventProjector
{
    async fn execute<I>(&self, id: I, cmd: ProductCommand) -> Result<(), Report<ApplicationError>>
        where 
            I: Into<Option<ProductId>> + Sync + Send,
    {
        let manager = self.process_manager();
        
        let refs = if let ProductCommand::Register { .. } = &cmd {
            let id = ProductId::default();
            
            let product = Product::try_from((id, cmd.clone()))
                .change_context_lazy(|| ApplicationError::Formation)?;
            
            manager.spawn(id, product, 0).await
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
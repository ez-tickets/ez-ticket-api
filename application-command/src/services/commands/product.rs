use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::process::registry::ProcessSystem;

use crate::adapter::{DependOnEventProjector, DependOnProcessExtension, DependOnProcessRegistry};
use crate::errors::ApplicationError;
use kernel::commands::ProductCommand;
use kernel::entities::{Product, ProductId};

impl<T> ProductRegisterService for T
where
    T: DependOnEventProjector
     + DependOnProcessRegistry
     + DependOnProcessExtension
{}


pub trait DependOnProductRegisterService: 'static + Sync + Send {
    type ProductRegisterService: ProductRegisterService;
    fn product_register_service(&self) -> &Self::ProductRegisterService;
}

#[async_trait]
pub trait ProductRegisterService: 'static + Sync + Send 
where
    Self: DependOnEventProjector
        + DependOnProcessRegistry
        + DependOnProcessExtension
{
    async fn execute(&self, cmd: ProductCommand) -> Result<ProductId, Report<ApplicationError>> {
        if let ProductCommand::Register { name } = &cmd {
            let id = ProductId::default();
            let new = Product::new(id, name.clone());

            let refs = self.process_registry()
                .spawn(id, new, 0, self.process_extension())
                .await
                .change_context_lazy(|| ApplicationError::Other)?;

            let event = refs.publish(cmd).await
                .change_context_lazy(|| ApplicationError::Other)?
                .change_context_lazy(|| ApplicationError::Kernel)?;
            
            refs.apply(event).await
                .change_context_lazy(|| ApplicationError::Other)?;

            return Ok(id)
        }
        
        Err(Report::new(ApplicationError::InvalidGivenCommand)
            .attach_printable("This service only accepts `ProductCommand::Register`."))
    }
}




#[rustfmt::skip]
impl<T> ProductCommandExecutor for T 
where 
    T: DependOnEventProjector
     + DependOnProcessRegistry
     + DependOnProcessExtension
{}


pub trait DependOnProductCommandExecutor: 'static + Sync + Send {
    type ProductCommandExecutor: ProductCommandExecutor;
    fn product_command_executor(&self) -> &Self::ProductCommandExecutor;
}


#[rustfmt::skip]
#[async_trait]
pub trait ProductCommandExecutor: 'static + Sync + Send 
where 
    Self: DependOnEventProjector
        + DependOnProcessRegistry
        + DependOnProcessExtension
{
    async fn execute(&self, id: ProductId, cmd: ProductCommand) -> Result<(), Report<ApplicationError>> {
        if let ProductCommand::Register { .. } = cmd {
            return Err(Report::new(ApplicationError::InvalidGivenCommand)
                .attach_printable("This service denies `ProductCommand::Register`."))
        }
        
        
        let refs = match self.process_registry()
            .find::<Product>(id).await
            .change_context_lazy(|| ApplicationError::Other)?
        {
            Some(refs) => refs,
            None => {
                let (replay, seq) = self.projector()
                    .projection_to_latest::<Product>(id, None)
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

        refs.apply(event).await
            .change_context_lazy(|| ApplicationError::Other)?;

        Ok(())
    }
}
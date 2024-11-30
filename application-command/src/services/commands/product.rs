use crate::adapter::{DependOnProcessExtension, DependOnProcessRegistry, DependOnEventProjector};
use crate::errors::ApplicationError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::commands::ProductCommand;
use kernel::entities::{Product, ProductId};
use nitinol::process::registry::ProcessSystem;

#[rustfmt::skip]
impl<T> ProductCommandExecutor for T 
where 
    T: DependOnEventProjector
     + DependOnProcessRegistry
     + DependOnProcessExtension{}


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
    async fn execute(&self, id: Option<ProductId>, cmd: ProductCommand) -> Result<(), Report<ApplicationError>> {
        if let ProductCommand::Create { name, desc, price } = &cmd {
            let id = ProductId::default();
            let new = Product::create(id, name.clone(), desc.clone(), *price);

            let refs = self.process_registry()
                .spawn(id, new, 0, self.process_extension())
                .await
                .change_context_lazy(|| ApplicationError::Other)?;

            let event = refs.publish(cmd).await
                .change_context_lazy(|| ApplicationError::Other)?
                .change_context_lazy(|| ApplicationError::Kernel)?;


            refs.apply(event).await
                .change_context_lazy(|| ApplicationError::Other)?;

            return Ok(())
        }

        let Some(product) = id else {
            return Err(Report::new(ApplicationError::MissingId {
                entity: "Product"
            }))
        };

        let refs = match self.process_registry()
            .find::<Product>(product).await
            .change_context_lazy(|| ApplicationError::Other)?
        {
            Some(refs) => refs,
            None => {
                let (replay, seq) = self.projector()
                    .projection_to_latest::<Product>(product, None)
                    .await
                    .change_context_lazy(|| ApplicationError::Other)?;

                self.process_registry()
                    .spawn(product, replay, seq, self.process_extension())
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
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::process::registry::ProcessSystem;
use kernel::commands::CatalogCommand;
use kernel::entities::{Catalog, CatalogId};
use crate::adapter::{DependOnProcessExtension, DependOnProcessRegistry};
use crate::errors::ApplicationError;


impl<T> CreateCatalogService for T
where
    T: DependOnProcessRegistry
     + DependOnProcessExtension
{}

pub trait DependOnCreateCatalogService: 'static + Sync + Send {
    type CreateCatalogService: CreateCatalogService;
    fn create_catalog_service(&self) -> &Self::CreateCatalogService;
}

#[async_trait]
pub trait CreateCatalogService: 'static + Sync + Send
where
    Self: DependOnProcessRegistry
        + DependOnProcessExtension
{
    async fn execute(&self, cmd: CatalogCommand) -> Result<CatalogId, Report<ApplicationError>> {
        if let CatalogCommand::Create { name, desc, price, main } = cmd.clone() {
            let id = CatalogId::default();
            let catalog = Catalog::create(id, name, desc, price, main);
            
            let refs = self.process_registry()
                .spawn(id, catalog, 0, self.process_extension())
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
            .attach_printable("This service only accepts `CatalogCommand::Create`."))
    }
}
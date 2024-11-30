use std::ops::Deref;
use crate::errors::UnrecoverableError;
use driver::database::{init_sqlite, setup_eventstore};
use error_stack::{Report, ResultExt};
use nitinol::process::extension::Extensions as ProcessExtension;
use nitinol::process::persistence::extension::PersistenceExtension;
use nitinol::process::registry::Registry as ProcessRegistry;
use std::sync::Arc;
use nitinol::projection::Projector as EventProjector;
use nitinol::protocol::io::ReadProtocol;
use application_command::adapter::{DependOnEventProjector, DependOnProcessExtension, DependOnProcessRegistry};
use application_command::services::commands::{DependOnCategoriesCommandExecutor, DependOnCategoryCommandExecutor, DependOnProductCommandExecutor};
use application_query::adaptor::DependOnEventQueryProjector;
use application_query::models::DependOnCategoryQueryService;

pub struct AppModule {
    inner: Arc<Handler>
}

impl Clone for AppModule {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl Deref for AppModule {
    type Target = Handler;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

pub struct Handler {
    registry: ProcessRegistry,
    extension: ProcessExtension,
    projector: EventProjector,
}

impl Handler {
    pub async fn setup() -> Result<AppModule, Report<UnrecoverableError>> {
        let _pool = init_sqlite("sqlite://.database/level.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let event_store = setup_eventstore("sqlite://.database/journal.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let reader = ReadProtocol::new(event_store.clone());

        let registry = ProcessRegistry::default();
        let mut installer = ProcessExtension::builder();

        installer.install(PersistenceExtension::new(event_store.clone()))
            .change_context_lazy(|| UnrecoverableError)?;

        let extension = installer.build();

        let projector = EventProjector::new(reader);

        Ok(AppModule {
            inner: Arc::new(Handler {
                registry,
                extension,
                projector,
            })
        })
    }
}

impl DependOnEventProjector for Handler {
    fn projector(&self) -> &EventProjector {
        &self.projector
    }
}

impl DependOnProcessRegistry for Handler {
    fn process_registry(&self) -> ProcessRegistry {
        self.registry.clone()
    }
}

impl DependOnProcessExtension for Handler {
    fn process_extension(&self) -> ProcessExtension {
        self.extension.clone()
    }
}

impl DependOnCategoryCommandExecutor for Handler {
    type CategoryCommandExecutor = Handler;

    fn category_command_executor(&self) -> &Self::CategoryCommandExecutor {
        self
    }
}

impl DependOnCategoriesCommandExecutor for Handler {
    type CategoriesCommandExecutor = Handler;
    fn categories_command_executor(&self) -> &Self::CategoriesCommandExecutor {
        self
    }
}

impl DependOnProductCommandExecutor for Handler {
    type ProductCommandExecutor = Handler;

    fn product_command_executor(&self) -> &Self::ProductCommandExecutor {
        self
    }
}

impl DependOnEventQueryProjector for Handler {
    fn projector(&self) -> &EventProjector {
        &self.projector
    }
}

impl DependOnCategoryQueryService for Handler {
    type CategoryQueryService = Handler;

    fn category_query_service(&self) -> &Self::CategoryQueryService {
        self
    }
}
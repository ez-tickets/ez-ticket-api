use std::sync::Arc;
use std::ops::Deref;

use error_stack::{Report, ResultExt};
use nitinol::process::extension::Extensions as ProcessExtension;
use nitinol::process::persistence::extension::PersistenceExtension;
use nitinol::process::registry::Registry as ProcessRegistry;
use nitinol::projection::Projector as EventProjector;
use nitinol::protocol::io::ReadProtocol;
use application_command::{
    adapter::{
        DependOnEventProjector,
        DependOnProcessExtension,
        DependOnProcessRegistry
    },
    services::{
        commands::{
            DependOnCategoriesCommandExecutor,
            DependOnCategoryCommandExecutor,
            DependOnProductCommandExecutor,
            DependOnProductRegisterService,
            DependOnCreateCategoryService
        },
        content::{
            DependOnContentDeleteService,
            DependOnContentRegisterService,
            DependOnContentUpdateService
        }
    }
};
use application_query::{
    adaptor::DependOnEventQueryProjector,
    models::DependOnCategoryQueryService
};
use application_query::models::DependOnProductQueryService;
use driver::database::{init_sqlite, mkdir_if_none, setup_eventstore};
use driver::repositories::ImageDataBase;
use kernel::repositories::DependOnImageRepository;

use crate::errors::UnrecoverableError;

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
    image: ImageDataBase,
    
    registry: ProcessRegistry,
    extension: ProcessExtension,
    projector: EventProjector,
}

impl AppModule {
    pub async fn setup() -> Result<AppModule, Report<UnrecoverableError>> {
        mkdir_if_none().change_context_lazy(|| UnrecoverableError)?;
        
        let pool = init_sqlite("sqlite://.database/level.db").await
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
                image: ImageDataBase::new(pool),
                registry,
                extension,
                projector,
            })
        })
    }
}

// --- Base ---------------------------------------

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


// --- Command Module -----------------------------

impl DependOnCreateCategoryService for Handler {
    type CreateCategoryService = Handler;
    fn create_category_service(&self) -> &Self::CreateCategoryService {
        self
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

impl DependOnProductRegisterService for Handler {
    type ProductRegisterService = Handler;
    fn product_register_service(&self) -> &Self::ProductRegisterService {
        self
    }
}

impl DependOnProductCommandExecutor for Handler {
    type ProductCommandExecutor = Handler;
    fn product_command_executor(&self) -> &Self::ProductCommandExecutor {
        self
    }
}

impl DependOnContentRegisterService for Handler {
    type ContentRegisterService = Self;

    fn content_register_service(&self) -> &Self::ContentRegisterService {
        self
    }
}

impl DependOnContentUpdateService for Handler {
    type ContentUpdateService = Self;
    
    fn content_update_service(&self) -> &Self::ContentUpdateService {
        self
    }
}

impl DependOnContentDeleteService for Handler {
    type ContentDeleteService = Self;
    
    fn content_delete_service(&self) -> &Self::ContentDeleteService {
        self
    }
}

// --- Repository Module --------------------------
impl DependOnImageRepository for Handler {
    type ImageRepository = ImageDataBase;

    fn image_repository(&self) -> &Self::ImageRepository {
        &self.image
    }
}


// --- Query Module -------------------------------

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

impl DependOnProductQueryService for Handler {
    type ProductQueryService = Handler;

    fn product_query_service(&self) -> &Self::ProductQueryService {
        self
    }
}
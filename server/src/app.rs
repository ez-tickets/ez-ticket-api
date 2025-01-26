use std::sync::Arc;
use std::ops::Deref;

use error_stack::{Report, ResultExt};
use nitinol::eventstream::EventStream;
use nitinol::process::eventstream::EventStreamExtension;
use nitinol::process::manager::ProcessManager;
use nitinol::process::persistence::PersistenceExtension;
use nitinol::projection::EventProjector;
use nitinol::protocol::adapter::sqlite::SqliteEventStore;
use app_cmd::adapter::{DependOnEventProjector, DependOnProcessManager};
use app_cmd::services::categories::DependOnCategoriesCommandService;
use app_cmd::services::category::DependOnCategoryCommandService;
use app_cmd::services::product::DependOnProductCommandService;
use app_query::models::{
    DependOnGetAllCategoriesQueryService, 
    DependOnGetAllProductQueryService, 
    DependOnGetProductImageQueryService, 
    DependOnGetProductQueryService
};
use driver::database::{CategoryQueryModelService, ProductReadModelService};
use driver::database::query::{CategoryQueryService, ProductQueryService};
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
    manager: ProcessManager,
    projector: EventProjector,
    query_category: CategoryQueryService,
    query_product: ProductQueryService,
}

impl AppModule {
    pub async fn setup() -> Result<AppModule, Report<UnrecoverableError>> {
        let query = driver::database::init("sqlite:./.database/query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let eventstore = SqliteEventStore::setup("sqlite:./.database/journal.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let eventstream = EventStream::default();
        
        eventstream.subscribe(CategoryQueryModelService::new(query.clone())).await;
        eventstream.subscribe(ProductReadModelService::new(query.clone())).await;
        
        let manager = ProcessManager::with_extension(|ext| {
            ext.install(PersistenceExtension::new(eventstore.clone()))?
                .install(EventStreamExtension::new(eventstream))
        }).change_context_lazy(|| UnrecoverableError)?;
        
        let projector = EventProjector::new(eventstore);
        
        let query_category = CategoryQueryService::new(query.clone());
        let query_product = ProductQueryService::new(query);

        Ok(AppModule {
            inner: Arc::new(Handler {
                manager,
                projector,
                query_category,
                query_product,
            })
        })
    }
}

impl DependOnProcessManager for Handler {
    fn process_manager(&self) -> &ProcessManager {
        &self.manager
    }
}

impl DependOnEventProjector for Handler {
    fn event_projector(&self) -> &EventProjector {
        &self.projector
    }
}

impl DependOnCategoryCommandService for Handler {
    type CategoryCommandService = Self;

    fn category_command_service(&self) -> &Self::CategoryCommandService {
        self
    }
}

impl DependOnCategoriesCommandService for Handler {
    type CategoriesCommandService = Self;

    fn categories_command_service(&self) -> &Self::CategoriesCommandService {
        self
    }
}

impl DependOnProductCommandService for Handler {
    type ProductCommandService = Self;

    fn product_command_service(&self) -> &Self::ProductCommandService {
        self
    }
}

impl DependOnGetAllCategoriesQueryService for Handler {
    type GetAllCategoriesQueryService = CategoryQueryService;

    fn get_all_categories_query_service(&self) -> &Self::GetAllCategoriesQueryService {
        &self.query_category
    }
}

impl DependOnGetAllProductQueryService for Handler {
    type GetAllProductQueryService = ProductQueryService;

    fn get_all_product_query_service(&self) -> &Self::GetAllProductQueryService {
        &self.query_product
    }
}

impl DependOnGetProductQueryService for Handler {
    type GetProductQueryService = ProductQueryService;

    fn get_product_query_service(&self) -> &Self::GetProductQueryService {
        &self.query_product
    }
}

impl DependOnGetProductImageQueryService for Handler {
    type GetProductImageQueryService = ProductQueryService;

    fn get_product_image_query_service(&self) -> &Self::GetProductImageQueryService {
        &self.query_product
    }
}
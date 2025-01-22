use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use nitinol::eventstream::EventSubscriber;
use nitinol::eventstream::resolver::{DecodeMapping, SubscriptionMapper};
use sqlx::{SqliteConnection, SqlitePool};
use kernel::io::events::{CategoriesEvent, CategoryEvent};
use crate::errors::FailedBuildReadModel;

#[derive(Clone)]
pub struct CategoryQueryModelService {
    pool: SqlitePool
}

impl CategoryQueryModelService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl SubscriptionMapper for CategoryQueryModelService {
    fn mapping(mapping: &mut DecodeMapping<Self>) {
        mapping
            .register::<CategoryEvent>()
            .register::<CategoriesEvent>();
    }
}

#[async_trait]
impl EventSubscriber<CategoryEvent> for CategoryQueryModelService {
    type Error = Report<FailedBuildReadModel>;

    async fn on(&mut self, event: CategoryEvent) -> Result<(), Self::Error> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| FailedBuildReadModel)?;
        match event {
            CategoryEvent::Created { .. } => { 
                InternalCategoryQueryModelService::create_category(event, &mut con).await? 
            }
            CategoryEvent::Renamed { .. } => {
                InternalCategoryQueryModelService::rename_category(event, &mut con).await?
            }
            CategoryEvent::Deleted { .. } => {
                InternalCategoryQueryModelService::delete_category(event, &mut con).await?
            }
            CategoryEvent::AddedProduct { .. } => {}
            CategoryEvent::RemovedProduct { .. } => {}
            CategoryEvent::ChangedProductOrdering { .. } => {}
        }
        Ok(())
    }
}

#[async_trait]
impl EventSubscriber<CategoriesEvent> for CategoryQueryModelService {
    type Error = Report<FailedBuildReadModel>;

    async fn on(&mut self, event: CategoriesEvent) -> Result<(), Self::Error> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| FailedBuildReadModel)?;
        match event {
            CategoriesEvent::AddedCategory { .. } => { 
                InternalCategoryQueryModelService::register_category(event, &mut con).await? 
            }
            CategoriesEvent::ChangedOrdering { .. } => {
                InternalCategoryQueryModelService::change_ordering_category(event, &mut con).await?
            }
            _ => {}
        }
        Ok(())
    }
}

pub(crate) struct InternalCategoryQueryModelService;

impl InternalCategoryQueryModelService {
    pub async fn create_category(
        create: CategoryEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoryEvent::Created { id, name } = create else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO categories(id, name) VALUES (?, ?)
        "#)
            .bind(id.as_ref())
            .bind(name.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn register_category(
        create: CategoriesEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoriesEvent::AddedCategory { id, ordering } = create else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO categories_ordering(category, ordering) VALUES (?, ?)
        "#)
            .bind(id.as_ref())
            .bind(ordering)
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn rename_category(
        update: CategoryEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoryEvent::Renamed { id, new } = update else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            UPDATE categories SET name = ? WHERE id = ?
        "#)
            .bind(new.as_ref())
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn delete_category(
        delete: CategoryEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoryEvent::Deleted { id } = delete else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            DELETE FROM categories WHERE id = ?
        "#)
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn change_ordering_category(
        update: CategoriesEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoriesEvent::ChangedOrdering { new } = update else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            -- noinspection SqlWithoutWhereForFile
            DELETE FROM categories_ordering;
        "#)
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        for (category, ordering) in new {
            // language=sqlite
            sqlx::query(r#"
                INSERT INTO categories_ordering(category, ordering) VALUES (?, ?)
            "#)
                .bind(category)
                .bind(ordering.as_ref())
                .execute(&mut *con)
                .await
                .change_context_lazy(|| FailedBuildReadModel)?;
        }
        
        Ok(())
    }
    
    pub async fn add_product(
        add: CategoryEvent, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let CategoryEvent::AddedProduct { id, category, ordering } = add else {
            return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO category_products_ordering(product, category, ordering) VALUES (?, ?, ?)
        "#)
            .bind(id.as_ref())
            .bind(category.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
}
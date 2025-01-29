use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::io::events::{CategoriesEvent, CategoryEvent};
use nitinol::eventstream::resolver::{DecodeMapping, SubscriptionMapper};
use nitinol::eventstream::EventSubscriber;
use sqlx::{QueryBuilder, SqliteConnection, SqlitePool};
use sqlx::types::Uuid;
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
        let mut con = self.pool.begin().await
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
            CategoryEvent::AddedProduct { .. } => {
                InternalCategoryQueryModelService::add_product(event, &mut con).await?
            }
            CategoryEvent::RemovedProduct { .. } | 
            CategoryEvent::ChangedProductOrdering { .. } => {
                InternalCategoryQueryModelService::invalidate_ordering_product(event, &mut con).await?
            }
        }
        con.commit().await
            .change_context_lazy(|| FailedBuildReadModel)?;
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
            CategoriesEvent::ChangedOrdering { .. } | 
            CategoriesEvent::RemovedCategory { .. } => {
                InternalCategoryQueryModelService::invalidate_ordering_category(event, &mut con).await?
            }
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
    
    
    pub async fn invalidate_ordering_category(
        event: CategoriesEvent,
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let new = match event { 
            CategoriesEvent::ChangedOrdering { new } | 
            CategoriesEvent::RemovedCategory { new } => new,
            _ => return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type")),
        };
        
        // language=sqlite
        sqlx::query(r#"
            -- noinspection SqlWithoutWhereForFile
            DELETE FROM categories_ordering;
        "#)
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;


        let mut query = QueryBuilder::new("INSERT INTO categories_ordering(category, ordering) ");
        
        query.push_values(new, |mut q, (order, category)| {
            q.push_bind::<Uuid>(category.into())
                .push_bind(order);
        });
        
        query.build()
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
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
            .bind(ordering)
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn invalidate_ordering_product(
        event: CategoryEvent,
        con: &mut SqliteConnection
    ) -> Result<(), Report<FailedBuildReadModel>> {
        let (category, new) = match event { 
            CategoryEvent::RemovedProduct { category, new } |
            CategoryEvent::ChangedProductOrdering { category, new } => (category, new),
            _ => return Err(Report::new(FailedBuildReadModel)
                .attach_printable("Invalid event type")),
        };
        
        // language=sqlite
        sqlx::query(r#"
            DELETE FROM category_products_ordering WHERE category = ?;
        "#)
            .bind(category.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;

        let mut query = QueryBuilder::new("INSERT INTO category_products_ordering(product, category, ordering) ");
        
        query.push_values(new, |mut q, (order, product)| {
            q.push_bind::<Uuid>(product.into())
                .push_bind(category.as_ref())
                .push_bind(order);
        });
        
        query.build()
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use error_stack::Report;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use kernel::entities::category::{CategoryId, CategoryName};
    use kernel::entities::product::ProductId;
    
    use super::*;
    use crate::errors::test::UnrecoverableError;

    async fn create_category(id: CategoryId, ordering: i64, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let create_category_event = CategoryEvent::Created { id, name: CategoryName::new("Test").unwrap() };
        let register_category_event = CategoriesEvent::AddedCategory { id, ordering, };

        InternalCategoryQueryModelService::create_category(create_category_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        InternalCategoryQueryModelService::register_category(register_category_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    #[tokio::test]
    async fn test_create_category() -> Result<(), Report<UnrecoverableError>> {
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let category_id = CategoryId::default();

        create_category(category_id, 0, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    async fn rename_category(id: CategoryId, new: CategoryName, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let rename_category_event = CategoryEvent::Renamed { id, new };

        InternalCategoryQueryModelService::rename_category(rename_category_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    #[tokio::test]
    async fn test_rename_category() -> Result<(), Report<UnrecoverableError>> {
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let category_id = CategoryId::default();

        create_category(category_id, 0, &mut transaction).await?;
        rename_category(category_id, CategoryName::new("Test 2").unwrap(), &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    async fn delete_category(id: CategoryId, invalidate: BTreeMap<i64, CategoryId>, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let delete_category_event = CategoryEvent::Deleted { id };
        let delete_categories_event = CategoriesEvent::RemovedCategory { new: invalidate };

        InternalCategoryQueryModelService::delete_category(delete_category_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        InternalCategoryQueryModelService::invalidate_ordering_category(delete_categories_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    // noinspection DuplicatedCode
    #[tokio::test]
    async fn test_delete_category() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let category_id = CategoryId::default();

        let sets = vec![
            (0, category_id),
            (1, CategoryId::default()),
            (2, CategoryId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, CategoryId>>();

        for (order, category) in sets.clone() {
            create_category(category, order, &mut transaction).await?;
        }

        let new = sets.iter()
            .filter(|(_, exist)| *exist != &category_id)
            .enumerate()
            .map(|(idx, (_, id))| (idx as i64, *id))
            .collect();
        
        delete_category(category_id, new, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    async fn change_ordering_category(new: BTreeMap<i64, CategoryId>, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let change_ordering_category_event = CategoriesEvent::ChangedOrdering { new };

        InternalCategoryQueryModelService::invalidate_ordering_category(change_ordering_category_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    #[tokio::test]
    async fn test_change_ordering_category() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let sets = vec![
            (0, CategoryId::default()),
            (1, CategoryId::default()),
            (2, CategoryId::default()),
            (3, CategoryId::default()),
            (4, CategoryId::default()),
            (5, CategoryId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, CategoryId>>();

        for (ordering, id) in sets.clone() {
            create_category(id, ordering, &mut transaction).await?;
        }

        let orderings = sets.clone().into_keys().rev().collect::<Vec<i64>>();
        let categories = sets.clone().into_values().collect::<Vec<CategoryId>>();

        let new = orderings.into_iter()
            .zip(categories.into_iter())
            .collect::<BTreeMap<i64, CategoryId>>();

        change_ordering_category(new, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    async fn add_product(
        id: ProductId, 
        category: CategoryId, 
        ordering: i64, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<UnrecoverableError>> {
        let add_product_event = CategoryEvent::AddedProduct { id, category, ordering };
        
        crate::database::product::test::register_product(id, con).await?;
        InternalCategoryQueryModelService::add_product(add_product_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    #[tokio::test]
    async fn test_add_product() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let product_id = ProductId::default();
        let category_id = CategoryId::default();

        create_category(category_id, 0, &mut transaction).await?;
        add_product(product_id, category_id, 0, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    async fn remove_product(
        invalidate: BTreeMap<i64, ProductId>,
        category: CategoryId,
        con: &mut SqliteConnection
    ) -> Result<(), Report<UnrecoverableError>> {
        let remove_product_event = CategoryEvent::RemovedProduct { category, new: invalidate };

        InternalCategoryQueryModelService::invalidate_ordering_product(remove_product_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    // noinspection DuplicatedCode
    #[tokio::test]
    async fn test_remove_product() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let product_id = ProductId::default();
        let category_id = CategoryId::default();

        create_category(category_id, 0, &mut transaction).await?;

        let sets = vec![
            (0, product_id),
            (1, ProductId::default()),
            (2, ProductId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, ProductId>>();

        for (ordering, product_id) in sets.clone() {
            add_product(product_id, category_id, ordering, &mut transaction).await?;
        }

        let new = sets.iter()
            .filter(|(_, exist)| *exist != &product_id)
            .enumerate()
            .map(|(idx, (_, id))| (idx as i64, *id))
            .collect();
        
        remove_product(new, category_id, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    
    async fn change_ordering_product(
        category: CategoryId, 
        new: BTreeMap<i64, ProductId>, 
        con: &mut SqliteConnection
    ) -> Result<(), Report<UnrecoverableError>> {
        let change_ordering_product_event = CategoryEvent::ChangedProductOrdering { category, new };

        InternalCategoryQueryModelService::invalidate_ordering_product(change_ordering_product_event, con).await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }
    
    // noinspection DuplicatedCode
    #[tokio::test]
    async fn test_change_ordering_product() -> Result<(), Report<UnrecoverableError>> {
        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let category_id = CategoryId::default();

        create_category(category_id, 0, &mut transaction).await?;
        
        let sets = vec![
            (0, ProductId::default()),
            (1, ProductId::default()),
            (2, ProductId::default()),
            (3, ProductId::default()),
            (4, ProductId::default()),
            (5, ProductId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, ProductId>>();
        
        for (ordering, product_id) in sets.clone() {
            add_product(product_id, category_id, ordering, &mut transaction).await?;
        }

        let orderings = sets.clone().into_keys().rev().collect::<Vec<i64>>();
        let products = sets.clone().into_values().collect::<Vec<ProductId>>();

        let new = orderings.into_iter()
            .zip(products.into_iter())
            .collect::<BTreeMap<i64, ProductId>>();

        change_ordering_product(category_id, new, &mut transaction).await?;

        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;

        Ok(())
    }

    // noinspection DuplicatedCode
    #[tokio::test]
    async fn test_all() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();


        let con = crate::database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;

        let mut transaction = con.begin().await
            .change_context_lazy(|| UnrecoverableError)?;

        let category_id = CategoryId::default();

        let categories = vec![
            (0, category_id),
            (1, CategoryId::default()),
            (2, CategoryId::default()),
            (3, CategoryId::default()),
            (4, CategoryId::default()),
            (5, CategoryId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, CategoryId>>();

        for (ordering, id) in categories.clone() {
            create_category(id, ordering, &mut transaction).await?;
        }

        let orderings = categories.clone().into_keys().rev().collect::<Vec<i64>>();
        let categories = categories.clone().into_values().collect::<Vec<CategoryId>>();

        let new_categories = orderings.into_iter()
            .zip(categories.into_iter())
            .collect::<BTreeMap<i64, CategoryId>>();

        change_ordering_category(new_categories.clone(), &mut transaction).await?;

        rename_category(category_id, CategoryName::new("Test 2").unwrap(), &mut transaction).await?;
        
        let product_id = ProductId::default();
        
        let products = vec![
            (0, product_id),
            (1, ProductId::default()),
            (2, ProductId::default()),
            (3, ProductId::default()),
            (4, ProductId::default()),
            (5, ProductId::default()),
        ].into_iter()
            .collect::<BTreeMap<i64, ProductId>>();

        for (ordering, product_id) in products.clone() {
            add_product(product_id, category_id, ordering, &mut transaction).await?;
        }

        let orderings = products.clone().into_keys().rev().collect::<Vec<i64>>();
        let products = products.clone().into_values().collect::<Vec<ProductId>>();

        let new = orderings.into_iter()
            .zip(products.into_iter())
            .collect::<BTreeMap<i64, ProductId>>();

        change_ordering_product(category_id, new.clone(), &mut transaction).await?;

        let new_products = new.iter()
            .filter(|(_, exist)| *exist != &product_id)
            .enumerate()
            .map(|(idx, (_, id))| (idx as i64, *id))
            .collect();
        
        remove_product(new_products, category_id, &mut transaction).await?;

        let new_categories = new_categories.iter()
            .filter(|(_, exist)| *exist != &category_id)
            .enumerate()
            .map(|(idx, (_, id))| (idx as i64, *id))
            .collect();
        
        delete_category(category_id, new_categories, &mut transaction).await?;
        
        
        transaction.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
}
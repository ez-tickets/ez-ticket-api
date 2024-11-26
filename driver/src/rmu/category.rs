use application_query::errors::QueryError;
use application_query::models::{Category, CategoryQueryService};
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use sqlx::{Pool, Sqlite};
use std::collections::BTreeSet;
use nitinol::process::{Context, TryApplicator};
use kernel::events::{CategoriesEvent, CategoryEvent};

#[derive(Debug, Clone)]
pub struct CategoryRMU {
    pool: Pool<Sqlite>
}

impl CategoryRMU {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryQueryService for CategoryRMU {
    async fn find_all_category(&self) -> Result<BTreeSet<Category>, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError)?;
        
        // language=SQL
        let found = sqlx::query_as::<_, Category>(r#"
            SELECT
                id,
                name,
                ordering
            FROM categories
            JOIN categories_ordering co ON categories.id = co.category
            ORDER BY ordering
        "#)
            .fetch_all(&mut *con)
            .await
            .change_context_lazy(|| QueryError)?;
        
        let sorted = found.into_iter()
            .collect::<BTreeSet<Category>>();
        
        Ok(sorted)
    }
}

#[async_trait]
impl TryApplicator<CategoryEvent> for CategoryRMU {
    type Rejection = sqlx::Error;
    
    #[rustfmt::skip]
    async fn try_apply(&mut self, event: CategoryEvent, _: &mut Context) -> Result<(), Self::Rejection> {
        let mut transaction = self.pool.begin().await?;
        match event {
            CategoryEvent::Created { id, name } => {
                // language=SQL
                sqlx::query(r#"
                    INSERT INTO categories (id, name)
                    VALUES (?, ?)
                "#)
                    .bind(id.as_ref())
                    .bind(name.as_ref())
                    .execute(&mut *transaction)
                    .await?;
            }
            CategoryEvent::UpdatedName { id, name } => {
                // language=SQL
                sqlx::query(r#"
                    UPDATE categories
                    SET name = ?
                    WHERE id = ?
                "#)
                    .bind(name.as_ref())
                    .bind(id.as_ref())
                    .execute(&mut *transaction)
                    .await?;
            }
            CategoryEvent::Deleted { id } => {
                // language=SQL
                sqlx::query(r#"
                    DELETE FROM categories WHERE id = ?
                "#)
                    .bind(id.as_ref())
                    .execute(&mut *transaction)
                    .await?;
            }
            CategoryEvent::AddedProduct { id, product, ordering } => {
                // language=SQL
                sqlx::query(r#"
                    INSERT INTO products_ordering (category, product, ordering)
                    VALUES (?, ?, ?)
                "#)
                    .bind(id.as_ref())
                    .bind(product.as_ref())
                    .bind(ordering)
                    .execute(&mut *transaction).await?;
            }
            CategoryEvent::UpdatedProductOrdering { id, ordering } => {
                // language=sqlite
                sqlx::query(r#"
                    DELETE FROM products_ordering WHERE category = ?
                "#)
                    .bind(id.as_ref())
                    .execute(&mut *transaction)
                    .await?;
                
                for (ordering, product) in ordering {
                    // language=sqlite
                    sqlx::query(r#"
                        INSERT INTO products_ordering (category, product, ordering)
                        VALUES (?, ?, ?)
                    "#)
                        .bind(id.as_ref())
                        .bind(product.as_ref())
                        .bind(ordering)
                        .execute(&mut *transaction)
                        .await?;
                }
            }
            CategoryEvent::RemovedProduct { id, product } => {
                // language=sqlite
                sqlx::query(r#"
                    DELETE FROM products_ordering WHERE category = ? AND product = ?
                "#)
                    .bind(id.as_ref())
                    .bind(product.as_ref())
                    .execute(&mut *transaction)
                    .await?;
            }
        }
        transaction.commit().await?;
        Ok(())
    }
}

#[async_trait]
impl TryApplicator<CategoriesEvent> for CategoryRMU {
    type Rejection = sqlx::Error;
    
    #[rustfmt::skip]
    async fn try_apply(&mut self, event: CategoriesEvent, _: &mut Context) -> Result<(), Self::Rejection> {
        let mut transaction = self.pool.begin().await?;
        
        match event {
            CategoriesEvent::Added { id, ordering } => {
                // language=sqlite
                sqlx::query(r#"
                    INSERT INTO categories_ordering (category, ordering)
                    VALUES (?, ?)
                "#)
                    .bind(id.as_ref())
                    .bind(ordering)
                    .execute(&mut *transaction)
                    .await?;
            }
            CategoriesEvent::Removed { id } => {
                // language=sqlite
                sqlx::query(r#"
                    DELETE FROM categories_ordering WHERE category = ?
                "#)
                    .bind(id.as_ref())
                    .execute(&mut *transaction)
                    .await?;
            }
            CategoriesEvent::Updated { new } => {
                // language=sqlite
                sqlx::query(r#"
                    -- noinspection SqlWithoutWhereForFile
                    DELETE FROM categories_ordering
                "#)
                    .execute(&mut *transaction)
                    .await?;
                
                for (ordering, category) in new {
                    // language=SQL
                    sqlx::query(r#"
                        INSERT INTO categories_ordering (category, ordering)
                        VALUES (?, ?)
                    "#)
                        .bind(category.as_ref())
                        .bind(ordering)
                        .execute(&mut *transaction)
                        .await?;
                }
            }
        }
        transaction.commit().await?;
        Ok(())
    }
}
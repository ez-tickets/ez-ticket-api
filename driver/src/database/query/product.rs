use app_query::errors::QueryError;
use app_query::models::{AllProduct, GetAllProductQueryService, GetProductImageQueryService, GetProductQueryService, OrderedProduct, OrderedProducts, Product, ProductDetails};
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use sqlx::types::Uuid;
use std::collections::BTreeSet;

#[derive(Clone)]
pub struct ProductQueryService {
    pool: sqlx::SqlitePool,
}

impl ProductQueryService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GetAllProductQueryService for ProductQueryService {
    async fn get_all_product(&self) -> Result<AllProduct, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError::Driver)?;
        let all = InternalProductQueryService::get_all_product(&mut con).await
            .change_context_lazy(|| QueryError::Driver)?;
        Ok(all)
    }
    
    async fn get_all_product_by_category(&self, category: &Uuid) -> Result<OrderedProducts, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError::Driver)?;
        let all = InternalProductQueryService::get_all_product_by_category(&mut con, category).await
            .change_context_lazy(|| QueryError::Driver)?;
        Ok(all)
    }
}

#[async_trait]
impl GetProductQueryService for ProductQueryService {
    async fn get_product_details(&self, product: &Uuid) -> Result<ProductDetails, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError::Driver)?;
        let details = InternalProductQueryService::get_product_details(&mut con, product).await
            .change_context_lazy(|| QueryError::Driver)?;
        Ok(details)
    }
}

#[async_trait]
impl GetProductImageQueryService for ProductQueryService {
    async fn get_product_image(&self, id: &Uuid) -> Result<Vec<u8>, Report<QueryError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| QueryError::Driver)?;
        let image = InternalProductQueryService::get_product_image(&mut con, id).await
            .change_context_lazy(|| QueryError::Driver)?;
        Ok(image)
    }
}

pub(crate) struct InternalProductQueryService;

impl InternalProductQueryService {
    pub async fn get_all_product(con: &mut sqlx::SqliteConnection) -> Result<AllProduct, Report<QueryError>> {
        // language=sqlite
        let all = sqlx::query_as::<_, Product>(r#"
            SELECT 
                id, 
                name, 
                price
            FROM
                products
        "#)
            .fetch_all(&mut *con)
            .await
            .change_context_lazy(|| QueryError::Driver)?;
        
        Ok(AllProduct(all.into_iter().collect()))
    }
    
    pub async fn get_all_product_by_category(con: &mut sqlx::SqliteConnection, category: &Uuid) -> Result<OrderedProducts, Report<QueryError>> {
        // language=sqlite
        let all = sqlx::query_as::<_, OrderedProduct>(r#"
            SELECT 
                cpo.ordering,
                p.id, 
                p.name, 
                p.price
            FROM
                products p
            JOIN
                category_products_ordering cpo ON p.id = cpo.product
            WHERE
                cpo.category = ?
        "#)
            .bind(category)
            .fetch_all(&mut *con)
            .await
            .change_context_lazy(|| QueryError::Driver)?;
        
        let all = all.into_iter().collect::<BTreeSet<OrderedProduct>>();
        
        Ok(OrderedProducts(all))
    }
    
    pub async fn get_product_details(con: &mut sqlx::SqliteConnection, product: &Uuid) -> Result<ProductDetails, Report<QueryError>> {
        // language=sqlite
        let details = sqlx::query_as::<_, ProductDetails>(r#"
            SELECT 
                id, 
                name, 
                desc, 
                price
            FROM
                products
            WHERE
                id = ?
        "#)
            .bind(product)
            .fetch_one(&mut *con)
            .await
            .change_context_lazy(|| QueryError::Driver)?;
        
        Ok(details)
    }
    
    pub async fn get_product_image(con: &mut sqlx::SqliteConnection, id: &Uuid) -> Result<Vec<u8>, Report<QueryError>> {
        // language=sqlite
        let image = sqlx::query_scalar::<_, Vec<u8>>(r#"
            SELECT 
                image
            FROM
                images
            WHERE
                id = ?
        "#)
            .bind(id)
            .fetch_one(&mut *con)
            .await
            .change_context_lazy(|| QueryError::Driver)?;
        
        Ok(image)
    }
}
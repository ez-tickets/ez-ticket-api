use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::io::events::ProductEvent;
use nitinol::eventstream::resolver::{DecodeMapping, SubscriptionMapper};
use nitinol::eventstream::EventSubscriber;
use sqlx::{SqliteConnection, SqlitePool};

use crate::errors::FailedBuildReadModel;

#[derive(Clone)]
pub struct ProductReadModelService {
    pool: SqlitePool
}

impl ProductReadModelService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl SubscriptionMapper for ProductReadModelService {
    fn mapping(mapping: &mut DecodeMapping<Self>) {
        mapping.register::<ProductEvent>();
    }
}

#[async_trait]
impl EventSubscriber<ProductEvent> for ProductReadModelService {
    type Error = Report<FailedBuildReadModel>;

    async fn on(&mut self, event: ProductEvent) -> Result<(), Self::Error> {
        let mut con = self.pool.begin().await
            .change_context_lazy(|| FailedBuildReadModel)?;
        match event {
            ProductEvent::Registered { .. } => { 
                InternalProductReadModelService::create(event, &mut con).await? 
            }
            ProductEvent::RenamedProductName { .. } => { 
                InternalProductReadModelService::update_name(event, &mut con).await? 
            }
            ProductEvent::EditedProductDesc { .. } => {
                InternalProductReadModelService::update_desc(event, &mut con).await?
            }
            ProductEvent::ChangedProductPrice { .. } => {
                InternalProductReadModelService::update_price(event, &mut con).await?
            }
            ProductEvent::ChangedProductImage { .. } => {
                InternalProductReadModelService::update_image(event, &mut con).await?
            }
            ProductEvent::Deleted { .. } => {
                InternalProductReadModelService::delete(event, &mut con).await?
            }
        }
        Ok(())
    }
}


pub(crate) struct InternalProductReadModelService;

impl InternalProductReadModelService {
    pub async fn create(create: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::Registered { id, name, desc, price, image } = create else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO images(id, image) VALUES (?, ?)
        "#)
            .bind(image.id().as_ref())
            .bind::<&Vec<u8>>(image.image().as_ref())
            .execute(&mut *con).await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO products(id, name, image, desc, price) VALUES (?, ?, ?, ?, ?)
        "#)
            .bind(id.as_ref())
            .bind(name.as_ref())
            .bind(image.id().as_ref())
            .bind(desc.as_ref())
            .bind(price.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn update_name(update: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::RenamedProductName { id, new } = update else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            UPDATE products SET name = ? WHERE id = ?
        "#)
            .bind(new.as_ref())
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn update_desc(update: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::EditedProductDesc { id, new } = update else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            UPDATE products SET desc = ? WHERE id = ?
        "#)
            .bind(new.as_ref())
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn update_price(update: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::ChangedProductPrice { id, new } = update else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            UPDATE products SET price = ? WHERE id = ?
        "#)
            .bind(new.as_ref())
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn update_image(update: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::ChangedProductImage { id, image } = update else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            UPDATE images SET image = ? WHERE id = ?
        "#)
            .bind::<&Vec<u8>>(image.image().as_ref())
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
    
    pub async fn delete(delete: ProductEvent, con: &mut SqliteConnection) -> Result<(), Report<FailedBuildReadModel>> {
        let ProductEvent::Deleted { id } = delete else {
            return Err(Report::new(FailedBuildReadModel).attach_printable("Invalid event type"));
        };
        
        // language=sqlite
        sqlx::query(r#"
            DELETE FROM products WHERE id = ?
        "#)
            .bind(id.as_ref())
            .execute(&mut *con)
            .await
            .change_context_lazy(|| FailedBuildReadModel)?;
        
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use error_stack::{Report, ResultExt};
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use kernel::entities::image::{Image, ImageId};
    use kernel::entities::product::*;
    
    use super::*;
    use crate::database::{self};
    use crate::errors::test::UnrecoverableError;
    
    pub async fn register_product(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let image: Vec<u8> = include_bytes!("../../tests/resources/test_image.jpg").to_vec();
        
        let create = ProductEvent::Registered {
            id,
            name: ProductName::new("test"),
            desc: ProductDesc::new("test description"),
            price: ProductPrice::new(100).change_context_lazy(|| UnrecoverableError)?,
            image: Image::new(ImageId::from(id), image),
        };
        
        InternalProductReadModelService::create(create, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_register_product() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    pub async fn rename_product(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let update = ProductEvent::RenamedProductName {
            id,
            new: ProductName::new("new name"),
        };
        
        InternalProductReadModelService::update_name(update, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_rename_product() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        rename_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    pub async fn edit_product_desc(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let update = ProductEvent::EditedProductDesc {
            id,
            new: ProductDesc::new("new description"),
        };
        
        InternalProductReadModelService::update_desc(update, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_edit_product_desc() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        edit_product_desc(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    pub async fn change_product_price(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let update = ProductEvent::ChangedProductPrice {
            id,
            new: ProductPrice::new(200).change_context_lazy(|| UnrecoverableError)?,
        };
        
        InternalProductReadModelService::update_price(update, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_change_product_price() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        change_product_price(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    pub async fn change_product_image(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let image: Vec<u8> = include_bytes!("../../tests/resources/test_image.jpg").to_vec();
        
        let update = ProductEvent::ChangedProductImage {
            id,
            image: Image::new(ImageId::from(id), image),
        };
        
        InternalProductReadModelService::update_image(update, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_change_product_image() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        change_product_image(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    pub async fn delete_product(id: ProductId, con: &mut SqliteConnection) -> Result<(), Report<UnrecoverableError>> {
        let delete = ProductEvent::Deleted { id };
        
        InternalProductReadModelService::delete(delete, con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_delete_product() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        delete_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
    
    #[tokio::test]
    async fn test_all() -> Result<(), Report<UnrecoverableError>> {
        tracing_subscriber::registry()
            .with(EnvFilter::new("trace"))
            .with(tracing_subscriber::fmt::layer())
            .init();
        
        
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let product_id = ProductId::default();
        
        register_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        rename_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        edit_product_desc(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        change_product_price(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        change_product_image(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        delete_product(product_id, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
}
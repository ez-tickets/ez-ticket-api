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
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| FailedBuildReadModel)?;
        match event {
            ProductEvent::Registered { .. } => InternalProductReadModelService::create(event, &mut con).await?,
            ProductEvent::RenamedProductName { .. } => InternalProductReadModelService::update_name(event, &mut con).await?,
            ProductEvent::EditedProductDesc { .. } => {}
            ProductEvent::ChangedProductPrice { .. } => {}
            ProductEvent::Deleted { .. } => {}
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
            INSERT INTO products(id, name, image, desc, price) VALUES (?, ?, ?, ?, ?)
        "#)
            .bind(id.as_ref())
            .bind(name.as_ref())
            .bind(image.as_ref())
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
}

#[cfg(test)]
mod test {
    use error_stack::{Report, ResultExt};
    use kernel::entities::image::ImageId;
    use kernel::entities::product::*;
    
    use super::*;
    use crate::database::{self};
    use crate::errors::test::UnrecoverableError;
    
    async fn create_image(con: &mut SqliteConnection) -> Result<ImageId, Report<UnrecoverableError>> {
        let id = ImageId::default();
        let bin: Vec<u8> = vec![0x01];
        
        
        
        Ok(id)
    }

    #[tokio::test]
    async fn test_create_product() -> Result<(), Report<UnrecoverableError>> {
        let pool = database::init("sqlite:../query.db").await
            .change_context_lazy(|| UnrecoverableError)?;
        let mut con = pool.begin().await
            .change_context_lazy(|| UnrecoverableError)?;
        
        let create = ProductEvent::Registered {
            id: ProductId::default(),
            name: ProductName::new("test"),
            desc: ProductDesc::new("test"),
            price: ProductPrice::new(100).change_context_lazy(|| UnrecoverableError)?,
            image: ImageId::default(),
        };
        
        InternalProductReadModelService::create(create, &mut con).await
            .change_context_lazy(|| UnrecoverableError)?;
        
        
        con.rollback().await
            .change_context_lazy(|| UnrecoverableError)?;
        Ok(())
    }
}
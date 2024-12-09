use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use sqlx::{SqliteConnection, SqlitePool};
use kernel::entities::ImageId;
use kernel::errors::KernelError;
use kernel::repositories::ImageRepository;

#[derive(Debug, Clone)]
pub struct ImageDataBase {
    pool: SqlitePool
}

impl ImageDataBase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ImageRepository for ImageDataBase {
    async fn insert(&self, id: &ImageId, image: Vec<u8>) -> Result<(), Report<KernelError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| KernelError::Invalid)
            .attach_printable("cannot acquire connection")?;
        
        Internal::insert(id, image, &mut con).await
            .change_context_lazy(|| KernelError::Invalid)?;
        Ok(())
    }

    async fn update(&self, id: &ImageId, image: Vec<u8>) -> Result<(), Report<KernelError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| KernelError::Invalid)
            .attach_printable("cannot acquire connection")?;
        
        Internal::update(id, image, &mut con).await
            .change_context_lazy(|| KernelError::Invalid)?;
        
        Ok(())
    }

    async fn delete(&self, id: &ImageId) -> Result<(), Report<KernelError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| KernelError::Invalid)
            .attach_printable("cannot acquire connection")?;
        
        Internal::delete(id, &mut con).await
            .change_context_lazy(|| KernelError::Invalid)?;
        
        Ok(())
    }

    async fn select(&self, id: &ImageId) -> Result<Vec<u8>, Report<KernelError>> {
        let mut con = self.pool.acquire().await
            .change_context_lazy(|| KernelError::Invalid)
            .attach_printable("cannot acquire connection")?;
        
        Internal::select(id, &mut con).await
            .change_context_lazy(|| KernelError::Invalid)
    }
}

struct Internal;

#[derive(sqlx::FromRow)]
struct Image {
    image: Vec<u8>
}

impl Internal {
    pub async fn insert(id: &ImageId, image: Vec<u8>, con: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        // language=sqlite
        sqlx::query(r#"
            INSERT INTO images (id, image) VALUES (?, ?)
        "#)
            .bind(id.as_ref())
            .bind(image)
            .execute(&mut *con)
            .await?;
        
        Ok(())
    }

    pub async fn update(id: &ImageId, image: Vec<u8>, con: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        // language=sqlite
        sqlx::query(r#"
            UPDATE images SET image = ? WHERE id = ?
        "#)
            .bind(image)
            .bind(id.as_ref())
            .execute(&mut *con)
            .await?;
        
        Ok(())
    }

    pub async fn delete(id: &ImageId, con: &mut SqliteConnection) -> Result<(), sqlx::Error> {
        // language=sqlite
        sqlx::query(r#"
            DELETE FROM images WHERE id = ?
        "#)
            .bind(id.as_ref())
            .execute(&mut *con)
            .await?;
        
        Ok(())
    }

    pub async fn select(id: &ImageId, con: &mut SqliteConnection) -> Result<Vec<u8>, sqlx::Error> {
        // language=sqlite
        let bin = sqlx::query_as::<_, Image>(r#"
            SELECT image FROM images WHERE id = ?
        "#)
            .bind(id.as_ref())
            .fetch_one(&mut *con)
            .await?;
        
        Ok(bin.image)
    }
}
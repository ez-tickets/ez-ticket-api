use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::entities::ImageId;
use kernel::repositories::{DependOnImageRepository, ImageRepository};
use crate::errors::ApplicationError;

impl<T> ContentRegisterService for T 
where 
    T: DependOnImageRepository 
{}


pub trait DependOnContentRegisterService: 'static + Sync + Send {
    type ContentRegisterService: ContentRegisterService;
    fn content_register_service(&self) -> &Self::ContentRegisterService;
}

#[async_trait]
pub trait ContentRegisterService: 'static + Sync + Send 
where 
    Self: DependOnImageRepository
{
    async fn register_image(&self, id: ImageId, image: Vec<u8>) -> Result<(), Report<ApplicationError>> {
        self.image_repository()
            .insert(&id, image)
            .await
            .change_context_lazy(|| ApplicationError::Other)?;
        Ok(())
    }
}


impl<T> ContentUpdateService for T 
where 
    T: DependOnImageRepository 
{}

pub trait DependOnContentUpdateService: 'static + Sync + Send {
    type ContentUpdateService: ContentUpdateService;
    fn content_update_service(&self) -> &Self::ContentUpdateService;
}

#[async_trait]
pub trait ContentUpdateService: 'static + Sync + Send 
where
    Self: DependOnImageRepository
{
    async fn update_image(&self, id: ImageId, image: Vec<u8>) -> Result<(), Report<ApplicationError>> {
        self.image_repository()
            .update(&id, image)
            .await
            .change_context_lazy(|| ApplicationError::Other)?;
        Ok(())
    }
}


impl<T> ContentDeleteService for T 
where 
    T: DependOnImageRepository 
{}

pub trait DependOnContentDeleteService: 'static + Sync + Send {
    type ContentDeleteService: ContentDeleteService;
    fn content_delete_service(&self) -> &Self::ContentDeleteService;
}

#[async_trait]
pub trait ContentDeleteService: 'static + Sync + Send 
where 
    Self: DependOnImageRepository
{
    async fn delete_image(&self, id: ImageId) -> Result<(), Report<ApplicationError>> {
        self.image_repository()
            .delete(&id)
            .await
            .change_context_lazy(|| ApplicationError::Other)?;
        Ok(())
    }
}
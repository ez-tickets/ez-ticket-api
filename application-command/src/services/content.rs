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

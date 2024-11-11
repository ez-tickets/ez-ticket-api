use async_trait::async_trait;
use error_stack::Report;
use crate::entities::ImageId;
use crate::errors::KernelError;

#[async_trait]
pub trait ImageRepository: 'static + Sync + Send {
    async fn insert(&self, id: &ImageId, image: Vec<u8>) -> Result<(), Report<KernelError>>;
    async fn update(&self, id: &ImageId, image: Vec<u8>) -> Result<(), Report<KernelError>>;
    async fn delete(&self, id: &ImageId) -> Result<(), Report<KernelError>>;
    async fn select(&self, id: &ImageId) -> Result<Vec<u8>, Report<KernelError>>;
}

pub trait DependOnImageRepository: 'static + Sync + Send {
    type ImageRepository: ImageRepository;
    fn image_repository(&self) -> &Self::ImageRepository;
}
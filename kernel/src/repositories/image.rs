use crate::entities::image::{Image, ImageId};
use crate::errors::DriverError;
use async_trait::async_trait;

#[async_trait]
pub trait ImageRepository: 'static + Sync + Send {
    async fn insert(&self, img: &Image) -> Result<(), DriverError>;
    async fn update(&self, img: &Image) -> Result<(), DriverError>;
    async fn delete(&self, id: &ImageId) -> Result<(), DriverError>;
    async fn select(&self, id: &ImageId) -> Result<Option<Image>, DriverError>;
}

pub trait DependOnImageRepository: 'static + Sync + Send {
    type ImageRepository: ImageRepository;
    fn image_repository(&self) -> &Self::ImageRepository;
}

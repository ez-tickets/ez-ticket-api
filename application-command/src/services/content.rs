use async_trait::async_trait;
use error_stack::Report;
use kernel::repositories::DependOnImageRepository;
use crate::errors::ApplicationError;

#[async_trait]
pub trait ContentService: 'static + Sync + Send 
where 
    Self: DependOnImageRepository
{
    async fn execute(&self, _image: Vec<u8>) -> Result<(), Report<ApplicationError>> {
        Ok(())
    }
}
use async_trait::async_trait;

#[async_trait]
pub trait CategoryCommandExecutor: 'static + Sync + Send 
where 
    Self: // Add Dependencies
{
    
}
use async_trait::async_trait;

#[async_trait]
pub trait ProductCommandExecutor: 'static + Sync + Send 
where 
    Self: // Add Dependencies
{
    
}
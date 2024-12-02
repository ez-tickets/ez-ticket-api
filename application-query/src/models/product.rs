use crate::errors::QueryError;
use async_trait::async_trait;
use kernel::events::ProductEvent;
use nitinol::projection::Projection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: i32,
}

#[async_trait]
impl Projection<ProductEvent> for Product {
    type Rejection = QueryError;
    
    async fn first(event: ProductEvent) -> Result<Self, Self::Rejection> {
        if let ProductEvent::Created { id, name, desc, price } = event {
            return Ok(Product {
                id: id.into(),
                name: name.into(),
                description: desc.into(),
                price: price.into(),
            });
        }
        Err(QueryError)
    }
    
    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::UpdatedName { name } => {
                self.name = name.into();
            }
            ProductEvent::UpdatedDescription { desc } => {
                self.description = desc.into();
            }
            ProductEvent::UpdatedPrice { price } => {
                self.price = price.into();
            }
            ProductEvent::Deleted => {
                return Err(QueryError)
            },
            _ => return Ok(())
        }
        Ok(())
    }
}
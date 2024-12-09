mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use crate::commands::ProductCommand;
use crate::errors::KernelError;
use crate::events::ProductEvent;
use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::process::persistence::process::WithPersistence;
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
}

impl Product {
    pub fn new(id: ProductId, name: ProductName) -> Self {
        Self { id, name, }
    }
}

impl Product {
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }
}

impl ResolveMapping for Product {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<ProductEvent>();
    }
}

impl Process for Product {}

impl WithPersistence for Product {
    fn aggregate_id(&self) -> impl ToEntityId {
        self.id
    }
}

#[async_trait]
impl Publisher<ProductCommand> for Product {
    type Event = ProductEvent;
    type Rejection = KernelError;

    async fn publish(&self, command: ProductCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            ProductCommand::Register { name } => {
                ProductEvent::Created { id: self.id, name }
            }
            ProductCommand::UpdateName { name } => {
                let name = ProductName::new(name);
                ProductEvent::UpdatedName { id: self.id, name }
            }
            ProductCommand::Delete => {
                ProductEvent::Deleted { id: self.id }
            }
        };
        Ok(ev)
    }
}

#[async_trait]
impl Applicator<ProductEvent> for Product {
    async fn apply(&mut self, event: ProductEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        match event {
            ProductEvent::UpdatedName { name, .. } => {
                self.name = name;
            }
            ProductEvent::Deleted { .. } => {
                ctx.poison_pill().await;
            }
            _ => {}
        }
    }
}

#[async_trait]
impl Projection<ProductEvent> for Product {
    type Rejection = KernelError;
    
    async fn first(event: ProductEvent) -> Result<Self, Self::Rejection> {
        let ProductEvent::Created { id, name } = event else {
            return Err(KernelError::Invalid)
        };
        
        Ok(Self::new(id, name))
    }
    
    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::UpdatedName { name, .. } => {
                self.name = name;
            }
            ProductEvent::Deleted { .. } => {
                return Err(KernelError::Invalid)
            }
            _ => return Ok(())
        }
        Ok(())
    }
}
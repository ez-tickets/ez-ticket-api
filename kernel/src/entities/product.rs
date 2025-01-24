mod desc;
mod id;
mod name;
mod price;

pub use self::{desc::*, id::*, name::*, price::*};

use std::convert::Infallible;
use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use error_stack::Report;
use serde::{Deserialize, Serialize};

use nitinol::process::eventstream::WithStreamPublisher;
use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::resolver::{Mapper, ResolveMapping};
use nitinol::projection::Projection;
use nitinol::{EntityId, ToEntityId};
use crate::entities::image::{Image, ImageId};
use crate::errors::{FormationError, ValidationError};
use crate::io::commands::ProductCommand;
use crate::io::events::ProductEvent;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    desc: ProductDesc,
    price: ProductPrice,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        desc: ProductDesc,
        price: ProductPrice,
    ) -> Product {
        Product {
            id,
            name,
            desc,
            price,
        }
    }

    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn desc(&self) -> &ProductDesc {
        &self.desc
    }

    pub fn price(&self) -> &ProductPrice {
        &self.price
    }
}

impl TryFrom<(ProductId, ProductCommand)> for Product {
    type Error = Report<FormationError>;

    fn try_from(value: (ProductId, ProductCommand)) -> Result<Self, Self::Error> {
        let ProductCommand::Register { name, desc, price, .. } = value.1 else {
            return Err(Report::new(FormationError)
                .attach_printable("ProductCommand::Register is the only command that can be converted to Product", ));
        };
        
        Ok(Product { id: value.0, name, desc, price })
    }
}

impl Process for Product {}

impl WithPersistence for Product {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

impl WithStreamPublisher for Product {
    fn aggregate_id(&self) -> EntityId {
        self.id.to_entity_id()
    }
}

#[async_trait]
impl Publisher<ProductCommand> for Product {
    type Event = ProductEvent;
    type Rejection = Report<ValidationError>;

    async fn publish(
        &self,
        command: ProductCommand,
        _: &mut Context,
    ) -> Result<Self::Event, Self::Rejection> {
        match command {
            ProductCommand::Register { name, desc, price, image } => { 
                let image = Image::new(ImageId::from(self.id), image);
                Ok(ProductEvent::Registered { id: self.id, name, desc, price, image }) 
            },
            ProductCommand::RenameProductName { new } => {
                Ok(ProductEvent::RenamedProductName { id: self.id, new })
            }
            ProductCommand::EditProductDesc { new } => { 
                Ok(ProductEvent::EditedProductDesc { id: self.id, new }) 
            }
            ProductCommand::ChangeProductPrice { new } => {
                Ok(ProductEvent::ChangedProductPrice { id: self.id, new })
            }
            ProductCommand::ChangeProductImage { image } => {
                let image = Image::new(ImageId::from(self.id), image);
                Ok(ProductEvent::ChangedProductImage { id: self.id, image })
            }
            ProductCommand::Delete => { 
                Ok(ProductEvent::Deleted { id: self.id }) 
            }
        }
    }
}

#[async_trait]
impl Applicator<ProductEvent> for Product {
    async fn apply(&mut self, event: ProductEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        WithStreamPublisher::publish(self, &event, ctx).await;
        
        tracing::debug!("Applying event: {:?}", event);

        match event {
            ProductEvent::RenamedProductName { new, .. } => {
                self.name = new;
            }
            ProductEvent::EditedProductDesc { new, .. } => {
                self.desc = new;
            }
            ProductEvent::ChangedProductPrice { new, .. } => {
                self.price = new;
            }
            ProductEvent::Deleted { .. } => {
                ctx.poison_pill().await;
            }
            _ => {}
        }

        tracing::debug!("State: {:?}", self);
    }
}

impl ResolveMapping for Product {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<ProductEvent>();
    }
}

#[async_trait]
impl Projection<ProductEvent> for Product {
    type Rejection = Infallible;

    async fn first(event: ProductEvent) -> Result<Self, Self::Rejection> {
        let ProductEvent::Registered { id, name, desc, price, .. } = event else {
            panic!("Projection must start with `ProductCommand::Register` event");
        };

        Ok(Self::new(id, name, desc, price))
    }

    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::RenamedProductName { new, .. } => {
                self.name = new;
            }
            ProductEvent::EditedProductDesc { new, .. } => {
                self.desc = new;
            }
            ProductEvent::ChangedProductPrice { new, .. } => {
                self.price = new;
            }
            ProductEvent::Deleted { .. } => {
                panic!("This entity has a delete event issued.");
            }
            _ => {}
        }

        Ok(())
    }
}

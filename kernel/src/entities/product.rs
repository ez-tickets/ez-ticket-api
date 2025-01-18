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

use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::projection::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;

use crate::entities::image::ImageId;
use crate::errors::{FormationError, ValidationError};
use crate::io::commands::ProductCommand;
use crate::io::events::ProductEvent;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    desc: ProductDesc,
    price: ProductPrice,
    image: ImageId,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        desc: ProductDesc,
        price: ProductPrice,
        image: ImageId,
    ) -> Product {
        Product {
            id,
            name,
            desc,
            price,
            image,
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

    pub fn image(&self) -> &ImageId {
        &self.image
    }
}

impl TryFrom<(ProductId, ProductCommand)> for Product {
    type Error = Report<FormationError>;

    fn try_from(value: (ProductId, ProductCommand)) -> Result<Self, Self::Error> {
        let ProductCommand::Register {
            name,
            desc,
            price,
            image,
        } = value.1 else {
            return Err(Report::new(FormationError).attach_printable(
                "ProductCommand::Register is the only command that can be converted to Product",
            ));
        };
        
        Ok(Product { id: value.0, name, desc, price, image })
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
    type Rejection = Report<ValidationError>;

    async fn publish(
        &self,
        command: ProductCommand,
        _: &mut Context,
    ) -> Result<Self::Event, Self::Rejection> {
        match command {
            ProductCommand::Register {
                name,
                desc,
                price,
                image,
            } => Ok(ProductEvent::Registered {
                id: self.id,
                name,
                desc,
                price,
                image,
            }),
            ProductCommand::RenameProductName { new } => {
                Ok(ProductEvent::RenamedProductName { new })
            }
            ProductCommand::EditProductDesc { new } => Ok(ProductEvent::EditedProductDesc { new }),
            ProductCommand::ChangeProductPrice { new } => {
                Ok(ProductEvent::ChangedProductPrice { new })
            }
            ProductCommand::Delete => Ok(ProductEvent::Deleted),
        }
    }
}

#[async_trait]
impl Applicator<ProductEvent> for Product {
    async fn apply(&mut self, event: ProductEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;

        match event {
            ProductEvent::RenamedProductName { new } => {
                self.name = new;
            }
            ProductEvent::EditedProductDesc { new } => {
                self.desc = new;
            }
            ProductEvent::ChangedProductPrice { new } => {
                self.price = new;
            }
            ProductEvent::Deleted => {
                ctx.poison_pill().await;
            }
            _ => {}
        }
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
        let ProductEvent::Registered {
            id,
            name,
            desc,
            price,
            image,
        } = event
        else {
            panic!("Projection must start with `ProductCommand::Register` event");
        };

        Ok(Self::new(id, name, desc, price, image))
    }

    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::RenamedProductName { new } => {
                self.name = new;
            }
            ProductEvent::EditedProductDesc { new } => {
                self.desc = new;
            }
            ProductEvent::ChangedProductPrice { new } => {
                self.price = new;
            }
            ProductEvent::Deleted => {
                panic!("This entity has a delete event issued.");
            }
            _ => {}
        }

        Ok(())
    }
}

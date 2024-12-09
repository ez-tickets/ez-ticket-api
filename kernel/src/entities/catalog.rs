mod id;
mod name;
mod price;
mod desc;
mod main_prod;

pub use self::desc::*;
pub use self::id::*;
pub use self::main_prod::*;
pub use self::name::*;
pub use self::price::*;

use async_trait::async_trait;
use error_stack::Report;
use nitinol::process::persistence::process::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;
use serde::{Deserialize, Serialize};

use crate::commands::CatalogCommand;
use crate::errors::KernelError;
use crate::events::CatalogEvent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Catalog {
    id: CatalogId,
    name: CatalogName,
    desc: CatalogDesc,
    price: Price,
    main: MainProduct,
}

impl Catalog {
    pub fn create(
        id: CatalogId,
        name: CatalogName,
        desc: CatalogDesc,
        price: Price,
        main: MainProduct,
    ) -> Self {
        Self {
            id,
            name,
            desc,
            price,
            main,
        }
    }
}

impl Catalog {
    pub fn id(&self) -> &CatalogId {
        &self.id
    }

    pub fn name(&self) -> &CatalogName {
        &self.name
    }

    pub fn desc(&self) -> &CatalogDesc {
        &self.desc
    }

    pub fn price(&self) -> &Price {
        &self.price
    }

    pub fn main(&self) -> &MainProduct {
        &self.main
    }
}

impl ResolveMapping for Catalog {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CatalogEvent>();
    }
}

impl Process for Catalog {}

impl WithPersistence for Catalog {
    fn aggregate_id(&self) -> impl ToEntityId {
        self.id
    }
}

#[async_trait]
impl Publisher<CatalogCommand> for Catalog {
    type Event = CatalogEvent;
    type Rejection = Report<KernelError>;

    async fn publish(&self, command: CatalogCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            CatalogCommand::Create { name, desc, price, main } => {
                CatalogEvent::Created { id: self.id, name, desc, price, main }
            }
            CatalogCommand::UpdateName { name } => {
                CatalogEvent::UpdatedName { id: self.id, name }
            }
            CatalogCommand::UpdateDesc { desc } => {
                CatalogEvent::UpdatedDesc { id: self.id, desc }
            }
            CatalogCommand::UpdatePrice { price } => {
                CatalogEvent::UpdatedPrice { id: self.id, price }
            }
            CatalogCommand::Delete => {
                CatalogEvent::Deleted
            }
        };
        Ok(ev)
    }
}


#[async_trait]
impl Applicator<CatalogEvent> for Catalog {
    async fn apply(&mut self, event: CatalogEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        if Projection::apply(self, event).await.is_err() {
            ctx.poison_pill().await;
        }
    }
}


#[async_trait]
impl Projection<CatalogEvent> for Catalog {
    type Rejection = KernelError;

    async fn first(event: CatalogEvent) -> Result<Self, Self::Rejection> {
        if let CatalogEvent::Created { id, name, desc, price, main } = event {
            return Ok(Self::create(id, name, desc, price, main));
        }
        Err(KernelError::Invalid)
    }

    async fn apply(&mut self, event: CatalogEvent) -> Result<(), Self::Rejection> {
        match event {
            CatalogEvent::UpdatedName { name, .. } => {
                self.name = name;
            }
            CatalogEvent::UpdatedDesc { desc, .. } => {
                self.desc = desc;
            }
            CatalogEvent::UpdatedPrice { price, .. } => {
                self.price = price;
            }
            CatalogEvent::Deleted => {
                return Err(KernelError::Invalid);
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}
mod id;
mod name;

pub use self::{id::*, name::*};

use std::collections::{BTreeMap, HashSet};

use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use error_stack::Report;
use serde::{Deserialize, Serialize};

use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;

use crate::entities::product::ProductId;
use crate::errors::ValidationError;
use crate::process::commands::CategoryCommand;
use crate::process::events::CategoryEvent;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    products: BTreeMap<i32, ProductId>,
}

impl Category {
    pub fn new(id: CategoryId, name: CategoryName) -> Category {
        Category {
            id,
            name,
            products: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> &CategoryId {
        &self.id
    }

    pub fn name(&self) -> &CategoryName {
        &self.name
    }

    pub fn products(&self) -> &BTreeMap<i32, ProductId> {
        &self.products
    }
}

impl Process for Category {}

impl WithPersistence for Category {
    fn aggregate_id(&self) -> impl ToEntityId {
        self.id
    }
}

#[async_trait]
impl Publisher<CategoryCommand> for Category {
    type Event = CategoryEvent;
    type Rejection = Report<ValidationError>;

    async fn publish(
        &self,
        command: CategoryCommand,
        _: &mut Context,
    ) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            CategoryCommand::Create { name } => CategoryEvent::Created { id: self.id, name },
            CategoryCommand::Rename { new } => CategoryEvent::Renamed { new },
            CategoryCommand::Delete => CategoryEvent::Deleted,
            CategoryCommand::AddProduct { id } => {
                if self.products.iter().any(|(_, p)| p == &id) {
                    return Err(Report::new(ValidationError)
                        .attach_printable("Product already exists in category"));
                }

                CategoryEvent::AddedProduct { id }
            }
            CategoryCommand::RemoveProduct { id } => {
                if !self.products.iter().any(|(_, p)| p == &id) {
                    return Err(Report::new(ValidationError)
                        .attach_printable("Product does not exist in category"));
                }

                CategoryEvent::RemovedProduct { id }
            }
            CategoryCommand::ChangeProductOrdering { new } => {
                let older = self.products.values().copied().collect::<HashSet<_>>();
                let newer = new.values().copied().collect::<HashSet<_>>();

                if !(&older ^ &newer).is_empty() {
                    return Err(Report::new(ValidationError).attach_printable(
                        "Product ordering must not be added or deleted within this command",
                    ));
                }

                CategoryEvent::ChangedProductOrdering { new }
            }
        };

        Ok(ev)
    }
}

#[async_trait]
impl Applicator<CategoryEvent> for Category {
    async fn apply(&mut self, event: CategoryEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;

        match event {
            CategoryEvent::Created { name, .. } => {
                self.name = name;
            }
            CategoryEvent::Renamed { new } => {
                self.name = new;
            }
            CategoryEvent::Deleted => {
                ctx.poison_pill().await;
            }
            CategoryEvent::AddedProduct { id } => {
                let next = self.products.len() as i32;
                self.products.insert(next, id);
            }
            CategoryEvent::RemovedProduct { id } => {
                self.products.retain(|_, p| p != &id);
            }
            CategoryEvent::ChangedProductOrdering { new } => {
                self.products = new;
            }
        }
    }
}

impl ResolveMapping for Category {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CategoryEvent>();
    }
}

#[async_trait]
impl Projection<CategoryEvent> for Category {
    type Rejection = ValidationError;

    async fn first(event: CategoryEvent) -> Result<Self, Self::Rejection> {
        let CategoryEvent::Created { id, name } = event else {
            panic!("Projection must start with `CategoryEvent::Created` event");
        };
        Ok(Self::new(id, name))
    }

    async fn apply(&mut self, event: CategoryEvent) -> Result<(), Self::Rejection> {
        match event {
            CategoryEvent::Renamed { new } => {
                self.name = new;
            }
            CategoryEvent::Deleted => {
                panic!("This entity has a delete event issued.");
            }
            CategoryEvent::AddedProduct { id } => {
                let next = self.products.len() as i32;
                self.products.insert(next, id);
            }
            CategoryEvent::RemovedProduct { id } => {
                self.products.retain(|_, p| p != &id);
            }
            CategoryEvent::ChangedProductOrdering { new } => {
                self.products = new;
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}

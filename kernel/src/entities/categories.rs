use std::collections::{BTreeMap, HashSet};
use std::convert::Infallible;

use async_trait::async_trait;
use error_stack::Report;
use serde::{Deserialize, Serialize};

use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::projection::resolver::{Mapper, ResolveMapping};
use nitinol::{EntityId, ToEntityId};
use nitinol::process::eventstream::WithStreamPublisher;
use crate::entities::category::CategoryId;
use crate::errors::ValidationError;
use crate::io::commands::CategoriesCommand;
use crate::io::events::CategoriesEvent;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Categories {
    categories: BTreeMap<i64, CategoryId>,
}

impl Categories {
    pub const ID: &'static str = "categories";
    
    fn apply(&mut self, event: CategoriesEvent) {
        match event {
            CategoriesEvent::AddedCategory { id, ordering } => {
                self.categories.insert(ordering, id);
            }
            CategoriesEvent::RemovedCategory { new } | 
            CategoriesEvent::ChangedOrdering { new } => {
                self.categories = new;
            }
        }
    }
}

impl AsRef<BTreeMap<i64, CategoryId>> for Categories {
    fn as_ref(&self) -> &BTreeMap<i64, CategoryId> {
        &self.categories
    }
}

impl Process for Categories {}

impl WithPersistence for Categories {
    fn aggregate_id(&self) -> EntityId {
        Categories::ID.to_entity_id()
    }
}

impl WithStreamPublisher for Categories {
    fn aggregate_id(&self) -> EntityId {
        Categories::ID.to_entity_id()
    }
}

#[async_trait]
impl Publisher<CategoriesCommand> for Categories {
    type Event = CategoriesEvent;
    type Rejection = Report<ValidationError>;

    #[tracing::instrument(skip_all, name = "categories")]
    async fn publish(
        &self,
        command: CategoriesCommand,
        _: &mut Context,
    ) -> Result<Self::Event, Self::Rejection> {
        match command {
            CategoriesCommand::AddCategory { id } => {
                if self.categories.iter().any(|(_, exist)| exist.eq(&id)) {
                    return Err(Report::new(ValidationError)
                        .attach_printable(format!("Category={id} already exists")));
                }
                Ok(CategoriesEvent::AddedCategory { id, ordering: self.categories.len() as i64 })
            }
            CategoriesCommand::RemoveCategory { id } => {
                if !self.categories.iter().any(|(_, exist)| exist.eq(&id)) {
                    return Err(Report::new(ValidationError)
                        .attach_printable(format!("Category={id} does not exist")));
                }

                let new = self.categories.iter()
                    .filter(|(_, exist)| *exist != &id)
                    .enumerate()
                    .map(|(idx, (_, id))| (idx as i64, *id))
                    .collect();

                Ok(CategoriesEvent::RemovedCategory { new })
            }
            CategoriesCommand::ChangeOrdering { new } => {
                let older = self
                    .categories
                    .values()
                    .copied()
                    .collect::<HashSet<CategoryId>>();
                let newer = new.values().copied().collect::<HashSet<CategoryId>>();

                if !(&older ^ &newer).is_empty() {
                    return Err(Report::new(ValidationError).attach_printable(
                        "Categories cannot be added or deleted within this command",
                    ));
                }

                Ok(CategoriesEvent::ChangedOrdering { new })
            }
        }
    }
}

#[async_trait]
impl Applicator<CategoriesEvent> for Categories {
    #[tracing::instrument(skip_all, name = "categories")]
    async fn apply(&mut self, event: CategoriesEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        WithStreamPublisher::publish(self, &event, ctx).await;
        
        tracing::debug!("Applying event: {:?}", event);
        Categories::apply(self, event);
        tracing::debug!("State: {:?}", self);
    }
}

impl ResolveMapping for Categories {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CategoriesEvent>();
    }
}

#[async_trait]
impl Projection<CategoriesEvent> for Categories {
    type Rejection = Infallible;

    async fn apply(&mut self, event: CategoriesEvent) -> Result<(), Self::Rejection> {
        Categories::apply(self, event);
        Ok(())
    }
}

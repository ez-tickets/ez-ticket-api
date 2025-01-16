use std::collections::{BTreeMap, HashSet};
use std::convert::Infallible;

use async_trait::async_trait;
use error_stack::Report;
use serde::{Deserialize, Serialize};

use nitinol::process::persistence::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;

use crate::entities::category::CategoryId;
use crate::errors::ValidationError;
use crate::process::commands::CategoriesCommand;
use crate::process::events::CategoriesEvent;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Categories {
    categories: BTreeMap<i32, CategoryId>,
}

impl Categories {
    fn apply(&mut self, event: CategoriesEvent) {
        match event {
            CategoriesEvent::AddedCategory { id } => {
                let next = self.categories.len() as i32;
                self.categories.insert(next, id);
            }
            CategoriesEvent::RemovedCategory { id } => {
                self.categories.retain(|_, exist| exist != &id);
            }
            CategoriesEvent::ChangedOrdering { new } => {
                self.categories = new;
            }
        }
    }
}

impl AsRef<BTreeMap<i32, CategoryId>> for Categories {
    fn as_ref(&self) -> &BTreeMap<i32, CategoryId> {
        &self.categories
    }
}

impl Process for Categories {}

impl WithPersistence for Categories {
    fn aggregate_id(&self) -> impl ToEntityId {
        "categories"
    }
}

#[async_trait]
impl Publisher<CategoriesCommand> for Categories {
    type Event = CategoriesEvent;
    type Rejection = Report<ValidationError>;

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
                Ok(CategoriesEvent::AddedCategory { id })
            }
            CategoriesCommand::RemoveCategory { id } => {
                if !self.categories.iter().any(|(_, exist)| exist.eq(&id)) {
                    return Err(Report::new(ValidationError)
                        .attach_printable(format!("Category={id} does not exist")));
                }

                Ok(CategoriesEvent::RemovedCategory { id })
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
    async fn apply(&mut self, event: CategoriesEvent, ctx: &mut Context) {
        self.persist(&event, ctx).await;
        Categories::apply(self, event);
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

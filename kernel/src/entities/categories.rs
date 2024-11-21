use crate::commands::CategoriesCommand;
use crate::entities::CategoryId;
use crate::events::CategoriesEvent;
use nitinol::agent::{Applicator, Context, Publisher};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::Infallible;
use async_trait::async_trait;
use nitinol::projection::Projection;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Categories {
    ordering: BTreeMap<i32, CategoryId>
}

impl Categories {
    pub fn new(ordering: BTreeMap<i32, CategoryId>) -> Categories {
        Self { ordering }
    }
}

impl AsRef<BTreeMap<i32, CategoryId>> for Categories {
    fn as_ref(&self) -> &BTreeMap<i32, CategoryId> {
        &self.ordering
    }
}

#[async_trait]
impl Publisher<CategoriesCommand> for Categories {
    type Event = CategoriesEvent;
    type Rejection = ();

    async fn publish(&self, command: CategoriesCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            CategoriesCommand::Update { new } => {
                let new = new.into_iter()
                    .map(|(order, id)| (order, CategoryId::new(id)))
                    .collect::<BTreeMap<i32, CategoryId>>();
                CategoriesEvent::Updated { new }
            }
            CategoriesCommand::Add { id } => {
                if self.ordering.iter().any(|(_, v)| v == &id) {
                    return Err(());
                }
                CategoriesEvent::Added { id }
            }
            CategoriesCommand::Remove { id } => {
                if self.ordering.iter().any(|(_, v)| v != &id) {
                    return Err(());
                }
                CategoriesEvent::Removed { id }
            }
        };
        Ok(ev)
    }
}

#[async_trait]
impl Applicator<CategoriesEvent> for Categories {
    async fn apply(&mut self, event: CategoriesEvent, _: &mut Context) {
        Projection::apply(self, event).await.unwrap();
    }
}

#[async_trait]
impl Projection<CategoriesEvent> for Categories {
    type Rejection = Infallible;
    
    async fn apply(&mut self, event: CategoriesEvent) -> Result<(), Self::Rejection> {
        match event {
            CategoriesEvent::Updated { new } => {
                self.ordering = new;
            }
            CategoriesEvent::Added { id } => {
                self.ordering.insert(self.ordering.len() as i32, id);
            }
            CategoriesEvent::Removed { id } => {
                self.ordering.retain(|_, v| v != &id);
            }
        }
        Ok(())
    }
}

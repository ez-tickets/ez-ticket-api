use crate::commands::CategoriesCommand;
use crate::entities::CategoryId;
use crate::events::CategoriesEvent;
use nitinol::agent::{Context, Publisher};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl Default for Categories {
    fn default() -> Self {
        Self {
            ordering: BTreeMap::new()
        }
    }
}

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
                todo!()
            }
            CategoriesCommand::Remove { id } => {
                todo!()
            }
        };
        Ok(ev)
    }
}
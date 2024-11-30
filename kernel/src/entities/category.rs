mod id;
mod name;

pub use self::id::*;
pub use self::name::*;

use crate::commands::CategoryCommand;
use crate::entities::ProductId;
use crate::errors::KernelError;
use crate::events::CategoryEvent;
use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use error_stack::Report;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use nitinol::process::persistence::process::WithPersistence;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Category {
    id: CategoryId,
    name: CategoryName,
    products: BTreeMap<i32, ProductId>,
}

impl Category {
    pub fn new(
        id: CategoryId,
        name: CategoryName,
        products: BTreeMap<i32, ProductId>,
    ) -> Self {
        Self {
            id,
            name,
            products,
        }
    }

    pub fn create(id: CategoryId, name: CategoryName) -> Self {
        Self {
            id,
            name,
            products: BTreeMap::new(),
        }
    }
}

impl Category {
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

impl ResolveMapping for Category {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CategoryEvent>();
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
    type Rejection = Report<KernelError>;

    async fn publish(&self, command: CategoryCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            CategoryCommand::Create { name } => {
                CategoryEvent::Created { id: self.id, name }
            }
            CategoryCommand::UpdateName { name } => {
                let name = CategoryName::new(name);
                CategoryEvent::UpdatedName { id: self.id, name }
            }
            CategoryCommand::Delete => {
                CategoryEvent::Deleted { id: self.id }
            }
            CategoryCommand::AddProduct { product_id } => {
                if self.products().values().any(|exist| exist.eq(&product_id)) {
                    return Err(Report::new(KernelError::AlreadyExists {
                        entity: "Category",
                        id: product_id.to_string(),
                    }));
                }
                CategoryEvent::AddedProduct { 
                    id: self.id,
                    product: product_id, 
                    ordering: self.products().len() as i32 + 1 
                }
            }
            CategoryCommand::UpdateProductOrdering { ordering } => {
                let old = self.products()
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let new = ordering
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let diff = &old ^ &new;

                if !diff.is_empty() {
                    return Err(Report::new(KernelError::Invalid))
                }

                CategoryEvent::UpdatedProductOrdering { id: self.id, ordering }
            }
            CategoryCommand::RemoveProduct { product } => {
                if self.products().values().any(|exist| exist.ne(&product)) {
                    return Err(Report::new(KernelError::NotFound {
                        entity: "Category",
                        id: product.to_string(),
                    }));
                }
                CategoryEvent::RemovedProduct { id: self.id, product }
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
            CategoryEvent::Created { id, name } => {
                self.id = id;
                self.name = name;
            }
            CategoryEvent::UpdatedName { name, .. } => {
                self.name = name;
            }
            CategoryEvent::Deleted { .. } => {
                ctx.poison_pill().await;
            }
            CategoryEvent::AddedProduct { product, ordering, .. } => {
                self.products.insert(ordering, product);
            }
            CategoryEvent::UpdatedProductOrdering { ordering, .. } => {
                self.products = ordering;
            }
            CategoryEvent::RemovedProduct { product: product_id, .. } => {
                self.products.retain(|_, exist| exist == &product_id);
            }
        }
    }
}

#[async_trait]
impl Projection<CategoryEvent> for Category {
    type Rejection = KernelError;

    async fn first(event: CategoryEvent) -> Result<Self, Self::Rejection> {
        let CategoryEvent::Created { id, name } = event else { 
            return Err(KernelError::Invalid)
        };
        
        Ok(Self::create(id, name))
    }

    async fn apply(&mut self, event: CategoryEvent) -> Result<(), Self::Rejection> {
        match event {
            CategoryEvent::UpdatedName { name, .. } => {
                self.name = name;
            }
            CategoryEvent::Deleted { .. } => {
                return Err(KernelError::Invalid)
            }
            CategoryEvent::AddedProduct { product, ordering, .. } => {
                self.products.insert(ordering, product);
            }
            CategoryEvent::UpdatedProductOrdering { ordering, .. } => {
                self.products = ordering;
            }
            CategoryEvent::RemovedProduct { product, .. } => {
                self.products.retain(|_, exist| exist == &product);
            }
            _ => return Ok(())
        }
        Ok(())
    }
}

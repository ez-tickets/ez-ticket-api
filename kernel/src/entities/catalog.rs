mod id;
mod name;
mod price;
mod desc;
mod option_id;
mod main_prod;
mod sub_prod;
mod opt_prod;

pub use self::id::*;
pub use self::name::*;
pub use self::desc::*;
pub use self::price::*;
pub use self::main_prod::*;
pub use self::sub_prod::*;
pub use self::opt_prod::*;
pub use self::option_id::*;

use std::collections::HashSet;

use async_trait::async_trait;
use error_stack::Report;
use nitinol::process::persistence::process::WithPersistence;
use nitinol::process::{Applicator, Context, Process, Publisher};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use nitinol::ToEntityId;
use serde::{Deserialize, Serialize};

use crate::commands::CatalogCommand;
use crate::entities::ProductId;
use crate::errors::KernelError;
use crate::events::CatalogEvent;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Catalog {
    id: CatalogId,
    name: CatalogName,
    desc: CatalogDesc,
    price: Price,
    main: MainProduct,
    subs: SubProduct,
    opts: OptProduct
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
            subs: SubProduct::default(),
            opts: OptProduct::default(),
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

    pub fn subs(&self) -> &SubProduct {
        &self.subs
    }

    pub fn opts(&self) -> &OptProduct {
        &self.opts
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
            CatalogCommand::AddMainProd { ordering, main } => {
                if self.main.as_ref().values().any(|id| id.eq(&main)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("MainProduct({main}) already exists")));
                }
                
                CatalogEvent::AddedMainProd { id: self.id, ordering, main }
            }
            CatalogCommand::UpdateMainProdOrdering { ordering } => {
                let older = self.main.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let newer = ordering.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let diff = &older ^ &newer;
                if !diff.is_empty() { 
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable("Order changes do not accept addition/deletion of content elements."));
                }
                
                CatalogEvent::UpdatedMainProdOrdering { id: self.id, ordering }
            }
            CatalogCommand::RemoveMainProd { main } => {
                if self.main.as_ref().values().all(|id| !id.eq(&main)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("MainProduct({main}) does not exist")));
                }
                
                CatalogEvent::RemovedMainProd { id: self.id, main }
            }
            CatalogCommand::AddSubProd { ordering, sub } => {
                if self.subs.as_ref().values().any(|id| id.eq(&sub)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("SubProduct({sub}) already exists")));
                }
                
                CatalogEvent::AddedSubProd { id: self.id, ordering, sub }
            }
            CatalogCommand::UpdateSubProdOrdering { ordering } => {
                let older = self.subs.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let newer = ordering.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<ProductId>>();
                let diff = &older ^ &newer;
                
                if !diff.is_empty() {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable("Order changes do not accept addition/deletion of content elements."));
                }
                
                CatalogEvent::UpdatedSubProdOrdering { id: self.id, ordering }
            }
            CatalogCommand::RemoveSubProd { sub } => {
                if self.subs.as_ref().values().all(|id| !id.eq(&sub)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("SubProduct({sub}) does not exist")));
                }
                
                CatalogEvent::RemovedSubProd { id: self.id, sub }
            }
            CatalogCommand::AddOptProd { ordering, opt } => {
                if self.opts.as_ref().values().any(|id| id.eq(&opt)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("OptProduct({opt}) already exists")));
                }

                CatalogEvent::AddedOptProd { id: self.id, ordering, opt }
            }
            CatalogCommand::UpdateOptProdOrdering { ordering } => {
                let older = self.opts.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<OptionId>>();
                let newer = ordering.as_ref()
                    .values()
                    .copied()
                    .collect::<HashSet<OptionId>>();
                let diff = &older ^ &newer;

                if !diff.is_empty() {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable("Order changes do not accept addition/deletion of content elements."));
                }

                CatalogEvent::UpdatedOptProdOrdering { id: self.id, ordering }
            }
            CatalogCommand::RemoveOptProd { opt } => {
                if self.opts.as_ref().values().all(|id| !id.eq(&opt)) {
                    return Err(Report::new(KernelError::Invalid)
                        .attach_printable(format!("OptProduct({opt}) does not exist")));
                }

                CatalogEvent::RemovedOptProd { id: self.id, opt }
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
            CatalogEvent::AddedMainProd { ordering, main, .. } => {
                self.main.as_mut().insert(ordering, main);
            }
            CatalogEvent::UpdatedMainProdOrdering { ordering, .. } => {
                self.main = ordering;
            }
            CatalogEvent::RemovedMainProd { main, .. } => {
                self.main.as_mut().retain(|_, id| *id != main)
            }
            CatalogEvent::AddedSubProd { ordering, sub, .. } => {
                self.subs.as_mut().insert(ordering, sub);
            }
            CatalogEvent::UpdatedSubProdOrdering { ordering, .. } => {
                self.subs = ordering;
            }
            CatalogEvent::RemovedSubProd { sub, .. } => {
                self.subs.as_mut().retain(|_, id| *id != sub)
            }
            CatalogEvent::AddedOptProd { ordering, opt, .. } => {
                self.opts.as_mut().insert(ordering, opt);
            }
            CatalogEvent::UpdatedOptProdOrdering { ordering, .. } => {
                self.opts = ordering;
            }
            CatalogEvent::RemovedOptProd { opt, .. } => {
                self.opts.as_mut().retain(|_, id| *id != opt);
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}
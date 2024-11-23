mod description;
mod id;
mod name;
mod price;
mod stock;

pub use self::description::*;
pub use self::id::*;
pub use self::name::*;
pub use self::price::*;
pub use self::stock::*;

use crate::commands::ProductCommand;
use crate::errors::KernelError;
use crate::events::ProductEvent;
use async_trait::async_trait;
use destructure::{Destructure, Mutation};
use nitinol::process::{Applicator, Context, Process, ProcessContext, Publisher};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: ProductDescription,
    price: Price,
    stock: Stock,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: ProductName,
        description: ProductDescription,
        stock: Stock,
        price: Price,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            stock,
        }
    }
    
    pub fn create(
        id: ProductId, 
        name: ProductName, 
        description: ProductDescription, 
        price: Price
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            stock: Stock::new(0),
        }
    }
}

impl Product {
    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn description(&self) -> &ProductDescription {
        &self.description
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }

    pub fn price(&self) -> &Price {
        &self.price
    }
}

impl ResolveMapping for Product {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<ProductEvent>();
    }
}

impl Process for Product {}

#[async_trait]
impl Publisher<ProductCommand> for Product {
    type Event = ProductEvent;
    type Rejection = KernelError;

    async fn publish(&self, command: ProductCommand, _: &mut Context) -> Result<Self::Event, Self::Rejection> {
        let ev = match command {
            ProductCommand::Create { name, desc, price } => {
                ProductEvent::Created { id: self.id, name, desc, price }
            }
            ProductCommand::UpdateName { name } => {
                let name = ProductName::new(name);
                ProductEvent::UpdatedName { name }
            }
            ProductCommand::UpdateDescription { desc } => {
                let desc = ProductDescription::new(desc);
                ProductEvent::UpdatedDescription { desc }
            }
            ProductCommand::StockIn { amount } => {
                if (self.stock().as_ref() + amount) < 0 {
                    return Err(KernelError::Invalid)
                }
                ProductEvent::StockedIn { amount }
            }
            ProductCommand::StockOut { amount } => {
                if (self.stock().as_ref() - amount) < 0 {
                    return Err(KernelError::Invalid)
                }
                ProductEvent::StockedOut { amount }
            }
            ProductCommand::UpdatePrice { price } => {
                let price = Price::new(price);
                ProductEvent::UpdatedPrice { price }
            }
            
            ProductCommand::Delete => {
                ProductEvent::Deleted
            }
        };
        Ok(ev)
    }
}

#[async_trait]
impl Applicator<ProductEvent> for Product {
    async fn apply(&mut self, event: ProductEvent, ctx: &mut Context) {
        match event {
            ProductEvent::UpdatedName { name } => {
                self.name = name;
            }
            ProductEvent::UpdatedDescription { desc } => {
                self.description = desc;
            }
            ProductEvent::StockedIn { amount } => {
                self.stock.stock_in(amount);
            }
            ProductEvent::StockedOut { amount } => {
                self.stock.stock_out(amount);
            }
            ProductEvent::UpdatedPrice { price } => {
                self.price = price;
            }
            ProductEvent::Deleted => {
                ctx.poison_pill();
            }
            _ => {}
        }
    }
}

#[async_trait]
impl Projection<ProductEvent> for Product {
    type Rejection = KernelError;
    
    async fn first(event: ProductEvent) -> Result<Self, Self::Rejection> {
        let ProductEvent::Created { id, name, desc, price } = event else {
            return Err(KernelError::Invalid)
        };
        
        Ok(Self::create(id, name, desc, price))
    }
    
    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::UpdatedName { name } => {
                self.name = name;
            }
            ProductEvent::UpdatedDescription { desc } => {
                self.description = desc;
            }
            ProductEvent::StockedIn { amount } => {
                self.stock.stock_in(amount);
            }
            ProductEvent::StockedOut { amount } => {
                self.stock.stock_out(amount);
            }
            ProductEvent::UpdatedPrice { price } => {
                self.price = price;
            }
            ProductEvent::Deleted => {
                return Err(KernelError::Invalid)
            }
            _ => return Ok(())
        }
        Ok(())
    }
}
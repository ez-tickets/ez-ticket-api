mod category;
mod description;
mod id;
mod name;
mod options;
mod price;
mod stock;

pub use self::{category::*, description::*, id::*, name::*, options::*, price::*, stock::*};

use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};
use crate::errors::KernelError;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: ProductDescription,
    stock: Stock,
    price: Price,
    category: Category,
    options: Option<Vec<ProductOption>>,
}

impl Product {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        id: ProductId,
        name: ProductName,
        description: ProductDescription,
        stock: Stock,
        price: Price,
        category: CategoryId,
        options: Option<Vec<ProductId>>,
        category_looking: impl Fn(&CategoryId) -> Result<Category, KernelError>,
        options_looking: impl Fn(&Option<Vec<ProductId>>) -> Result<Option<Vec<ProductOption>>, KernelError>,
    ) -> Result<Product, KernelError> {
        let category = category_looking(&category)?;
        let options = options_looking(&options)?;

        Ok(Self {
            id,
            name,
            description,
            stock,
            price,
            category,
            options,
        })
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

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn options(&self) -> &Option<Vec<ProductOption>> {
        &self.options
    }
}
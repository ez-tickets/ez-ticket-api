mod category;
mod description;
mod id;
mod name;
mod options;
mod price;
mod stock;

pub use self::{category::*, description::*, id::*, name::*, options::*, price::*, stock::*};
use std::collections::HashSet;

use crate::entities::IngredientId;
use crate::errors::KernelError;
use destructure::{Destructure, Mutation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    description: ProductDescription,
    stock: Stock,
    price: Price,
    category: Category,
    options: Option<HashSet<ProductOption>>,
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
        options_looking: impl Fn(
            &Option<Vec<ProductId>>,
        ) -> Result<Option<HashSet<ProductOption>>, KernelError>,
        ingredients: HashSet<IngredientId>,
        ingredient_consume: impl Fn(&HashSet<IngredientId>) -> Result<(), KernelError>,
    ) -> Result<Product, KernelError> {
        ingredient_consume(&ingredients)?;

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

    pub fn options(&self) -> &Option<HashSet<ProductOption>> {
        &self.options
    }
}

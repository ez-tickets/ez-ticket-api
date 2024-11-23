mod option_id;

pub use option_id::*;

use crate::entities::ProductId;
use crate::errors::KernelError;
use destructure::{Destructure, Mutation};
use error_stack::Report;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize, Serialize, Destructure, Mutation)]
pub struct ProductOption {
    product: ProductId,
    options: HashSet<OptionId>,
}

impl ProductOption {
    pub fn new(product: ProductId, options: HashSet<OptionId>) -> Self {
        Self { product, options }
    }

    pub fn add(&mut self, option: OptionId) -> Result<(), Report<KernelError>> {
        if !self.options.insert(option) {
            return Err(Report::new(KernelError::AlreadyExists {
                entity: "Option",
                id: option.to_string(),
            }));
        }
        Ok(())
    }
}

impl ProductOption {
    pub fn product(&self) -> &ProductId {
        &self.product
    }

    pub fn options(&self) -> &HashSet<OptionId> {
        &self.options
    }
}


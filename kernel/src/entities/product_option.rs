mod option_id;

pub use option_id::*;

use crate::entities::ProductId;
use crate::errors::KernelError;
use destructure::{Destructure, Mutation};
use error_stack::Report;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

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
            return Err(Report::new(KernelError::AlreadyExists));
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct OptionId(Uuid);

impl OptionId {
    pub fn new(id: impl Into<Uuid>) -> OptionId {
        Self(id.into())
    }
}

impl AsRef<Uuid> for OptionId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<OptionId> for Uuid {
    fn from(value: OptionId) -> Self {
        value.0
    }
}

impl From<OptionId> for ProductId {
    fn from(value: OptionId) -> Self {
        ProductId::new(value.0)
    }
}

impl From<ProductId> for OptionId {
    fn from(value: ProductId) -> Self {
        OptionId::new(value)
    }
}

impl Default for OptionId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

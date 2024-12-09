use std::collections::BTreeMap;
use error_stack::Report;
use serde::{Deserialize, Serialize};
use crate::entities::ProductId;
use crate::errors::KernelError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MainProduct(BTreeMap<i32, ProductId>);

impl MainProduct {
    pub fn new(non_empty: impl FnOnce(&mut BTreeMap<i32, ProductId>)) -> Result<Self, Report<KernelError>> {
        let mut main_product = BTreeMap::new();
        non_empty(&mut main_product);
        
        if main_product.is_empty() { 
            return Err(Report::new(KernelError::Invalid)
                .attach_printable("Main product must not empty"));
        }
        
        Ok(Self(main_product))
    }
}

impl AsRef<BTreeMap<i32, ProductId>> for MainProduct {
    fn as_ref(&self) -> &BTreeMap<i32, ProductId> {
        &self.0
    }
}

impl AsMut<BTreeMap<i32, ProductId>> for MainProduct {
    fn as_mut(&mut self) -> &mut BTreeMap<i32, ProductId> {
        &mut self.0
    }
}

impl From<MainProduct> for BTreeMap<i32, ProductId> {
    fn from(main_product: MainProduct) -> Self {
        main_product.0
    }
}

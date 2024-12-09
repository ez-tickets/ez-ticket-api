use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::entities::ProductId;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubProduct(BTreeMap<i32, ProductId>);

impl SubProduct {
    pub fn new(subs: BTreeMap<i32, ProductId>) -> Self {
        Self(subs)
    }
}

impl AsRef<BTreeMap<i32, ProductId>> for SubProduct {
    fn as_ref(&self) -> &BTreeMap<i32, ProductId> {
        &self.0
    }
}

impl AsMut<BTreeMap<i32, ProductId>> for SubProduct {
    fn as_mut(&mut self) -> &mut BTreeMap<i32, ProductId> {
        &mut self.0
    }
}

impl From<SubProduct> for BTreeMap<i32, ProductId> {
    fn from(value: SubProduct) -> Self {
        value.0
    }
}

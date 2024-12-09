use crate::entities::OptionId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptProduct(BTreeMap<i32, OptionId>);

impl OptProduct {
    pub fn new(opts: BTreeMap<i32, OptionId>) -> OptProduct {
        Self(opts)
    }
}

impl AsRef<BTreeMap<i32, OptionId>> for OptProduct {
    fn as_ref(&self) -> &BTreeMap<i32, OptionId> {
        &self.0
    }
}

impl AsMut<BTreeMap<i32, OptionId>> for OptProduct {
    fn as_mut(&mut self) -> &mut BTreeMap<i32, OptionId> {
        &mut self.0
    }
}

impl From<OptProduct> for BTreeMap<i32, OptionId> {
    fn from(value: OptProduct) -> Self {
        value.0
    }
}

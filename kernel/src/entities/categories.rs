use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::entities::category::CategoryId;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Categories {
    categories: BTreeMap<i32, CategoryId>,
}

impl AsRef<BTreeMap<i32, CategoryId>> for Categories {
    fn as_ref(&self) -> &BTreeMap<i32, CategoryId> {
        &self.categories
    }
}

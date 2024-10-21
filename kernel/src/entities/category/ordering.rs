use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct CategoryOrdering(i32);

impl CategoryOrdering {
    pub fn new(ordering: impl Into<i32>) -> Self {
        Self(ordering.into())
    }
}

impl AsRef<i32> for CategoryOrdering {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<CategoryOrdering> for i32 {
    fn from(ordering: CategoryOrdering) -> Self {
        ordering.0
    }
}

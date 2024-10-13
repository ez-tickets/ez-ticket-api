use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Amount(i32);

impl Amount {
    pub fn new(amount: impl Into<i32>) -> Self {
        Self(amount.into())
    }
}

impl AsRef<i32> for Amount {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl From<Amount> for i32 {
    fn from(amount: Amount) -> Self {
        amount.0
    }
}

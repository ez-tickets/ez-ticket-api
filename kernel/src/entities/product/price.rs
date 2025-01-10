use error_stack::Report;
use serde::{Deserialize, Serialize};
use crate::errors::ValidationError;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ProductPrice(i64);

impl ProductPrice {
    pub fn new(price: impl Into<i64>) -> Result<ProductPrice, Report<ValidationError>> {
        let price = price.into();
        if price < 0 { 
            return Err(Report::new(ValidationError)
                .attach_printable("`ProductPrice` must be greater than or equal to zero"));
        }
        
        Ok(Self(price))
    }
}

impl AsRef<i64> for ProductPrice {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl From<ProductPrice> for i64 {
    fn from(price: ProductPrice) -> Self {
        price.0
    }
}

mod id;
mod options;
mod name;
mod category;
mod price;

pub use self::{
    id::*,
    name::*,
    options::*,
    category::*,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    id: ProductId,
    name: ProductName,
    
}
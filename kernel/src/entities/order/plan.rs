use serde::{Deserialize, Serialize};
use crate::entities::{CatalogId, OptProduct, Quantity, SubProduct};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    catalog: CatalogId,
    quantity: Quantity,
    subs: SubProduct,
    opts: OptProduct,
}
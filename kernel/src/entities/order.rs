mod id;
mod plan;
mod quantity;

pub use self::id::*;
pub use self::plan::*;
pub use self::quantity::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Order {
    id: OrderId,
    plan: Vec<Plan>,
}

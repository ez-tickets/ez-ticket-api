use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub struct Price(i32);
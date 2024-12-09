use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleCatalog {
    pub id: Uuid,
    pub name: String,
    pub price: i32,
    pub image: Uuid,
}


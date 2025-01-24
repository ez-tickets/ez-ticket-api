use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct OrderedCategory {
    pub ordering: i64,
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}


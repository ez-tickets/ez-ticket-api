use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct OrderedCategory {
    pub ordering: i64,
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}


use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct OrderedCategory {
    pub ordering: i64,
    pub id: Uuid,
    pub name: String,
}

impl Eq for OrderedCategory {}

impl PartialEq<Self> for OrderedCategory {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) || self.ordering.eq(&other.ordering)
    }
}

impl PartialOrd<Self> for OrderedCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedCategory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ordering.cmp(&other.ordering)
    }
}


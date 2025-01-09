use error_stack::Report;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    name: String
}

#[derive(Debug, Deserialize)]
pub struct FindCategory {
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryName {
    name: String
}


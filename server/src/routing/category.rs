use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use crate::AppModule;
use crate::routing::request::category::{CreateCategory, FindCategory, UpdateCategoryName};

pub async fn categories(
    State(app): State<AppModule>
) -> Result<Json<Categories>, StatusCode> {
    Ok(Json(categories))
}

pub async fn register(
    State(app): State<AppModule>,
    Json(req): Json<CreateCategory>
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::CREATED)
}

pub async fn update_name(
    State(app): State<AppModule>,
    Query(dest): Query<FindCategory>,
    Json(req): Json<UpdateCategoryName>
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

pub async fn delete(
    State(app): State<AppModule>,
    Query(dest): Query<FindCategory>
) -> Result<StatusCode, StatusCode> {
    Ok(StatusCode::OK)
}

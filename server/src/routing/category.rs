use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use application_command::services::commands::{CategoryCommandExecutor, DependOnCategoryCommandExecutor};
use application_query::models::{Categories, CategoryQueryService};
use crate::AppModule;
use crate::routing::request::category::CreateCategory;

pub async fn categories(
    State(app): State<AppModule>
) -> Result<Json<Categories>, StatusCode> {
    let categories = app.find_all_category().await
        .map_err(|e| {
            tracing::error!("Failed to find all categories: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(categories))
}

pub async fn register(
    State(app): State<AppModule>,
    Json(req): Json<CreateCategory>
) -> Result<StatusCode, StatusCode> {
    let req = req.try_into()
        .map_err(|e| {
            tracing::error!("Failed to convert request into command: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    app.category_command_executor()
        .execute(None, req)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        
    Ok(StatusCode::CREATED)
}

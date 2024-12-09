use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use application_command::services::commands::{CategoryCommandExecutor, CreateCategoryService, DependOnCategoryCommandExecutor, DependOnCreateCategoryService};
use application_query::models::{Categories, CategoryQueryService};
use kernel::commands::CategoryCommand;
use crate::AppModule;
use crate::routing::request::category::{CreateCategory, FindCategory, UpdateCategoryName};

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
    
    app.create_category_service()
        .create(req)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        
    Ok(StatusCode::CREATED)
}

pub async fn update_name(
    State(app): State<AppModule>,
    Query(dest): Query<FindCategory>,
    Json(req): Json<UpdateCategoryName>
) -> Result<StatusCode, StatusCode> {
    let req = req.try_into()
        .map_err(|e| {
            tracing::error!("Failed to convert request into command: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;
    
    app.category_command_executor()
        .execute(dest.id, req)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}

pub async fn delete(
    State(app): State<AppModule>,
    Query(dest): Query<FindCategory>
) -> Result<StatusCode, StatusCode> {
    app.category_command_executor()
        .execute(dest.id, CategoryCommand::Delete)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(StatusCode::OK)
}
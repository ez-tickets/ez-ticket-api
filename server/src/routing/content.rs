use axum::extract::{Multipart, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use application_command::services::content::{ContentUpdateService, DependOnContentUpdateService};
use kernel::repositories::{DependOnImageRepository, ImageRepository};
use crate::AppModule;
use crate::routing::request::content::ImageFindById;
use crate::routing::response::errors::ErrorResponse;

pub async fn image(
    Query(query): Query<ImageFindById>,
    State(app): State<AppModule>
) -> Result<impl IntoResponse, ErrorResponse> {
    let image = app.image_repository()
        .select(&query.id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find image by id: {:?}", e);
            ErrorResponse::Deserialization
        })?;
    
    let res = Response::builder()
        .header(CONTENT_TYPE, mime::IMAGE_PNG.as_ref())
        .status(StatusCode::OK)
        .body(image.into_response())
        .unwrap();
    Ok(res)
}

pub async fn update(
    State(app): State<AppModule>,
    Query(query): Query<ImageFindById>,
    mut parts: Multipart
) -> Result<StatusCode, ErrorResponse> {
    let mut image = Vec::new();
    while let Some(field) = parts.next_field().await? {
        let Some(name) = field.name() else {
            break;
        };

        if name == "image" && field.content_type().ok_or(ErrorResponse::Deserialization)?.eq(mime::IMAGE_PNG.as_ref()) {
            image = field.bytes().await?.into();
        }
    }
    
    if image.is_empty() {
        return Err(ErrorResponse::Deserialization);
    }
    
    app.content_update_service()
        .update_image(query.id, image)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            ErrorResponse::Deserialization
        })?;
    
    Ok(StatusCode::OK)
}
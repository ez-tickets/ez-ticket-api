use axum::extract::{Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
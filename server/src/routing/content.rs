use axum::extract::{Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::AppModule;
use crate::routing::request::content::ImageFindById;
use crate::routing::response::errors::ErrorResponse;

pub async fn image(
    Query(query): Query<ImageFindById>,
    State(app): State<AppModule>
) -> Result<impl IntoResponse, ErrorResponse> {
    let res = Response::builder()
        .header(CONTENT_TYPE, mime::IMAGE_PNG.as_ref())
        .status(StatusCode::OK)
        .body(Vec::<u8>::new().into_response())
        .unwrap();
    Ok(res)
}
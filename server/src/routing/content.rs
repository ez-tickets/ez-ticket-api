use axum::extract::{Multipart, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn upload_image(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let bytes = field.bytes().await.unwrap();
    }
    Ok("a".to_string())
}

pub async fn get_image(Query(id): Query<String>) -> Result<impl IntoResponse, StatusCode> {
    
    
    Ok(StatusCode::SERVICE_UNAVAILABLE)
}
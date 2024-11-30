use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub enum ErrorResponse {
    Deserialization,
    
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        #[allow(unreachable_patterns)]
        match self {
            Self::Deserialization => (StatusCode::BAD_REQUEST, "invalid request format").into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


impl From<MultipartError> for ErrorResponse {
    fn from(_value: MultipartError) -> Self {
        Self::Deserialization
    }
}
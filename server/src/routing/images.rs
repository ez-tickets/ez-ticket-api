use std::io::Cursor;

use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use image::{ImageFormat, ImageReader};

use app_query::models::{DependOnGetProductImageQueryService, GetProductImageQueryService};
use kernel::entities::image::ImageId;

use crate::AppModule;

pub async fn get(
    State(app): State<AppModule>,
    Path(image_id): Path<ImageId>
) -> Result<impl IntoResponse, StatusCode> {
    let image = match app.get_product_image_query_service()
        .get_product_image(image_id.as_ref())
        .await 
    {
        Ok(image) => image,
        Err(e) => {
            tracing::error!("Failed to get product image: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let Ok(reader) = ImageReader::new(Cursor::new(&image))
        .with_guessed_format() 
    else {
        tracing::error!("Failed to create image reader.");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    
    let mime = match reader.format() {
        Some(format) => {
            match format {
                ImageFormat::Png => mime::IMAGE_PNG,
                ImageFormat::Jpeg => mime::IMAGE_JPEG,
                _ => {
                    tracing::error!("Unsupported image format.");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        None => {
            tracing::error!("Failed to get image format.");
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };
    
    let res = Response::builder()
        .header(CONTENT_TYPE, mime.as_ref())
        .body(image.into_response())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(res)
}
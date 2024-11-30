use axum::extract::{Multipart, Query, State};
use axum::http::StatusCode;
use axum::Json;
use crate::AppModule;
use crate::routing::request::products::{ProductFilter, RegisterProduct};
use crate::routing::response::errors::ErrorResponse;
use crate::routing::response::product::AllProductIdResponse;

pub async fn product(
    Query(filter): Query<ProductFilter>,
    State(app): State<AppModule>
) -> Result<Json<AllProductIdResponse>, ErrorResponse> {
    todo!()
}

pub async fn product_register(
    State(app): State<AppModule>,
    mut parts: Multipart,
) -> Result<StatusCode, ErrorResponse> {
    let mut reg = RegisterProduct::default();
    while let Some(field) = parts.next_field().await? {
        if field.name().ok_or(ErrorResponse::Deserialization)?.eq("image") 
            && field.content_type().ok_or(ErrorResponse::Deserialization)?.eq(mime::IMAGE_PNG.as_ref()) {
            reg.image = field.bytes().await?.into();
            continue;
        } else if field.name().ok_or(ErrorResponse::Deserialization)?.eq("name") {  
            reg.name = field.text().await?;
            continue;
        } else if field.name().ok_or(ErrorResponse::Deserialization)?.eq("description") {
            reg.desc = field.text().await?;
            continue;
        } else if field.name().ok_or(ErrorResponse::Deserialization)?.eq("price") {
            reg.price = field.text().await?.parse().map_err(|_| ErrorResponse::Deserialization)?;
            continue;
        }
    }
    
    tracing::debug!("{:#?}", reg);
    

    Ok(StatusCode::OK)
}
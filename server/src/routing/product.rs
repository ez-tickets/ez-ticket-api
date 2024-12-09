use std::ops::Deref;
use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Json;
use application_command::services::commands::{ProductRegisterService, ProductCommandExecutor};
use application_command::services::content::{ContentRegisterService, DependOnContentRegisterService};
use application_query::models::{AllProducts, DependOnProductQueryService, ProductQueryService};
use kernel::commands::ProductCommand;
use kernel::entities::ProductName;
use crate::AppModule;
use crate::routing::request::products::RegisterProduct;
use crate::routing::response::errors::ErrorResponse;

pub async fn product(
    State(app): State<AppModule>
) -> Result<Json<AllProducts>, ErrorResponse> {
    app.product_query_service()
        .find_all()
        .await
        .map(Json)
        .map_err(|e| {
            tracing::error!("Failed to find all products: {:?}", e);
            ErrorResponse::Deserialization
        })
}

pub async fn register(
    State(app): State<AppModule>,
    mut parts: Multipart,
) -> Result<StatusCode, ErrorResponse> {
    let mut reg = RegisterProduct::default();
    while let Some(field) = parts.next_field().await? {
        let Some(name) = field.name() else {
            break;
        };

        match name {
            "name" => {
                reg.name = field.text().await?;
            },
            "image" => {
                if field.content_type().ok_or(ErrorResponse::Deserialization)?.eq(mime::IMAGE_PNG.as_ref()) {
                    reg.image = field.bytes().await?.into();
                }
            },
            _ => {}
        }
    }
    
    let id = ProductRegisterService::execute(app.deref(), ProductCommand::Register {
            name: ProductName::new(reg.name),
        })
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute command: {:?}", e);
            ErrorResponse::Deserialization
        })?;
    
    let res = app.clone()
        .content_register_service()
        .register_image(id.into(), reg.image)
        .await
        .map_err(|e| {
            tracing::error!("Failed to register image: {:?}", e);
            ErrorResponse::Deserialization
        });

    if res.is_err() {
        ProductCommandExecutor::execute(app.deref(), id, ProductCommand::Delete).await
            .map_err(|e| {
                tracing::error!("Failed to execute command: {:?}", e);
                ErrorResponse::Deserialization
            })?;
    }
    
    Ok(StatusCode::OK)
}
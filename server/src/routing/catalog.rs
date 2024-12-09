use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use crate::AppModule;
use crate::routing::request::catalog::{CreateCatalog, CreateCatalogBase, RawCreateCatalog};

pub async fn register(
    State(app): State<AppModule>,
    mut multipart: Multipart
) -> Result<StatusCode, StatusCode> {
    let mut req = RawCreateCatalog {
        base: None,
        image: None,
    };
    
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let Some(name) = field.name() else {
            break;
        };
        
        match name { 
            "base" => {
                if field.content_type().ok_or(StatusCode::BAD_REQUEST)?.eq(mime::APPLICATION_JSON.as_ref()) {
                    let base = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                    let base = serde_json::from_str::<CreateCatalogBase>(&base).map_err(|_| StatusCode::BAD_REQUEST)?;
                    req.base = Some(base);
                }
            },
            "image" => {
                if field.content_type().ok_or(StatusCode::BAD_REQUEST)?.eq(mime::IMAGE_PNG.as_ref()) { 
                    let image = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                    let image = image.to_vec();
                    req.image = Some(image);
                }
            }
            _ => {}
        }
    }
    
    let req = CreateCatalog {
        base: req.base.ok_or(StatusCode::BAD_REQUEST)?,
        image: req.image.ok_or(StatusCode::BAD_REQUEST)?,
    };
    Ok(StatusCode::OK)
}

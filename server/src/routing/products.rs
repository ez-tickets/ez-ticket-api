use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::Json;

use app_cmd::services::product::{DependOnProductCommandService, ProductCommandService};
use app_query::models::{
    AllProduct, 
    DependOnGetAllProductQueryService, 
    DependOnGetProductQueryService, 
    GetAllProductQueryService, 
    GetProductQueryService, 
    OrderedProducts, 
    ProductDetails
};

use kernel::entities::category::CategoryId;
use kernel::entities::product::ProductId;
use kernel::io::commands::ProductCommand;

use crate::AppModule;
use crate::routing::request::products::{PatchProduct, RegisterProduct};

pub async fn get_products_in_category(
    State(app): State<AppModule>,
    Path(category_id): Path<CategoryId>
) -> Result<Json<OrderedProducts>, StatusCode> {
    let res = match app.get_all_product_query_service()
        .get_all_product_by_category(category_id.as_ref())
        .await 
    {
        Ok(filtered) => filtered,
        Err(e) => {
            tracing::error!("Failed to get product by category: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(res))
}


pub async fn get_all_products(
    State(app): State<AppModule>,
) -> Result<Json<AllProduct>, StatusCode> {
    let res = match app.get_all_product_query_service()
        .get_all_product()
        .await
    {
        Ok(products) => products,
        Err(e) => {
            tracing::error!("Failed to get all products: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(res))
}


pub async fn product_details(
    State(app): State<AppModule>,
    Path(product_id): Path<ProductId>
) -> Result<Json<ProductDetails>, StatusCode> {
    let res = match app.get_product_query_service()
        .get_product_details(product_id.as_ref())
        .await
    {
        Ok(product) => product,
        Err(e) => {
            tracing::error!("Failed to get product details: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(res))
}


pub async fn register(
    State(app): State<AppModule>,
    multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    let cmd = match RegisterProduct::from_multipart(multipart).await
        .map(ProductCommand::try_from)
    {
        Ok(Ok(cmd)) => cmd,
        Ok(Err(e)) => {
            tracing::error!("Failed to validate product: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
        Err(e) => {
            tracing::error!("Failed to parse multipart: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    if let Err(e) = app.product_command_service()
        .execute(None, cmd)
        .await
    {
        tracing::error!("Failed to register product: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}


pub async fn patch(
    State(app): State<AppModule>,
    Path(product_id): Path<ProductId>,
    multipart: Multipart,
) -> Result<StatusCode, StatusCode> {
    let PatchProduct { 
        name, 
        desc, 
        price, 
        image 
    } = match PatchProduct::from_multipart(multipart).await { 
        Ok(req) => req,
        Err(e) => {
            tracing::error!("Failed to parse multipart: {:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    if let Some(name) = name {
        if let Err(e) = app.product_command_service()
            .execute(product_id, ProductCommand::RenameProductName { new: name })
            .await
        {
            tracing::error!("Failed to rename product name: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    if let Some(desc) = desc {
        if let Err(e) = app.product_command_service()
            .execute(product_id, ProductCommand::EditProductDesc { new: desc })
            .await
        {
            tracing::error!("Failed to change product desc: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    if let Some(price) = price {
        if let Err(e) = app.product_command_service()
            .execute(product_id, ProductCommand::ChangeProductPrice { new: price })
            .await
        {
            tracing::error!("Failed to change product price: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    if let Some(image) = image {
        if let Err(e) = app.product_command_service()
            .execute(product_id, ProductCommand::ChangeProductImage { image })
            .await
        {
            tracing::error!("Failed to change product image: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(StatusCode::OK)
}


pub async fn delete(
    State(app): State<AppModule>,
    Path(product_id): Path<ProductId>,
) -> Result<StatusCode, StatusCode> {
    if let Err(e) = app.product_command_service()
        .execute(product_id, ProductCommand::Delete)
        .await
    {
        tracing::error!("Failed to delete product: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    
    Ok(StatusCode::OK)
}
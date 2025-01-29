use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use app_cmd::services::product::{DependOnProductCommandService, ProductCommandService};
use app_query::models::{
    AllProduct,
    ProductDetails,
    DependOnGetAllProductQueryService,
    DependOnGetProductQueryService,
    GetAllProductQueryService,
    GetProductQueryService, 
};
use kernel::entities::product::ProductId;
use kernel::io::commands::ProductCommand;

use crate::AppModule;
use crate::routing::request::products::{PatchProduct, RegisterProduct, RegisterProductWithCategory};


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        get,
        path = "/products",
    )
)]
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


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        get,
        path = "/products/{product_id}",
        params(
            ("product_id" = Uuid, Path)
        )
    )
)]
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


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        post,
        path = "/products",
        request_body(
            content = RegisterProduct,
            content_type = "multipart/form-data"
        )
    )
)]
pub async fn register(
    State(app): State<AppModule>,
    Query(query): Query<RegisterProductWithCategory>,
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

    match query.category {
        None => {
            if let Err(e) = app.product_command_service()
                .execute(None, cmd)
                .await
            {
                tracing::error!("Failed to register product: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
        Some(dest) => {
            use app_cmd::workflow::product::{DependOnRegisterProductWithCategoryWorkflow, RegisterProductWithCategoryWorkflow};
            let app = app.register_product_with_category_workflow();
            if let Err(e) = RegisterProductWithCategoryWorkflow::execute(app, dest, cmd).await {
                tracing::error!("Failed to register product with category: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    
    Ok(StatusCode::CREATED)
}

#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        patch,
        path = "/products/{product_id}",
        params(
            ("product_id" = Uuid, Path)
        ),
        request_body(
            content = PatchProduct,
            content_type = "multipart/form-data"
        )
    )
)]
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



#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        delete,
        path = "/products/{product_id}",
        params(
            ("product_id" = Uuid, Path)
        )
    )
)]
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

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use app_cmd::services::category::{CategoryCommandService, DependOnCategoryCommandService};
use app_cmd::services::categories::{CategoriesCommandService, DependOnCategoriesCommandService};
use app_query::models::{AllCategories, DependOnGetAllCategoriesQueryService, DependOnGetAllProductQueryService, GetAllCategoriesQueryService, GetAllProductQueryService, OrderedProducts};

use kernel::entities::category::CategoryId;
use kernel::entities::product::ProductId;
use kernel::io::commands::CategoryCommand;

use crate::AppModule;
use crate::routing::request::categories::{
    AddProduct, 
    ChangeCategoryOrdering, 
    ChangeProductOrdering, 
    CreateCategory, 
    RenameCategory
};


#[cfg_attr(
    feature = "apidoc", 
    utoipa::path(
        get,
        path = "/categories",
    )
)]
pub async fn categories(
    State(app): State<AppModule>
) -> Result<Json<AllCategories>, StatusCode> {
    let categories = match app.get_all_categories_query_service()
        .get_all_categories()
        .await 
    {
        Ok(categories) => categories,
        Err(e) => {
            tracing::error!("failed to get all categories: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    
    Ok(Json(categories))
}


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        get,
        path = "/categories/{category_id}",
        params(
            ("category_id" = Uuid, Path)
        ),
    )
)]
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


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        post,
        path = "/categories",
        responses(
            (status = OK),
            (status = BAD_REQUEST),
            (status = INTERNAL_SERVER_ERROR)
        )
    )
)]
pub async fn create(
    State(app): State<AppModule>,
    Json(req): Json<CreateCategory>
) -> Result<StatusCode, StatusCode> {
    let cmd = req.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match CategoryCommandService::execute(app.category_command_service(), None, cmd).await { 
        Ok(_) => {}
        Err(e) => {
            tracing::error!("failed to register category: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(StatusCode::CREATED)
}


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        patch,
        path = "/categories/{category_id}",
        params(
            ("category_id" = Uuid, Path)
        ),
        responses(
            (status = OK),
            (status = BAD_REQUEST),
            (status = INTERNAL_SERVER_ERROR)
        )
    )
)]
pub async fn update_name(
    State(app): State<AppModule>,
    Path(category_id): Path<CategoryId>,
    Json(req): Json<RenameCategory>
) -> Result<StatusCode, StatusCode> {
    let cmd = req.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match CategoryCommandService::execute(app.category_command_service(), category_id, cmd).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("failed to update category name: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    Ok(StatusCode::OK)
}


#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        delete,
        path = "/categories/{category_id}",
        params(
            ("category_id" = Uuid, Path)
        ),
        responses(
            (status = OK),
            (status = INTERNAL_SERVER_ERROR)
        )
    )
)]
pub async fn delete(
    State(app): State<AppModule>,
    Path(dest): Path<CategoryId>
) -> Result<StatusCode, StatusCode> {
    match CategoryCommandService::execute(app.category_command_service(), dest, CategoryCommand::Delete).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("failed to delete category: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    Ok(StatusCode::OK)
}



#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        put,
        path = "/categories",
        responses(
            (status = NO_CONTENT),
            (status = BAD_REQUEST),
            (status = INTERNAL_SERVER_ERROR)
        )
    )
)]
pub async fn change_ordering(
    State(app): State<AppModule>,
    Json(req): Json<ChangeCategoryOrdering>
) -> Result<StatusCode, StatusCode> {
    let cmd = req.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if let Err(e) = CategoriesCommandService::execute(app.categories_command_service(), cmd).await {
        tracing::error!("failed to change category ordering: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::NO_CONTENT)
}



#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        post,
        path = "/categories/{category_id}",
        params(
            ("category_id" = Uuid, Path)
        ),
        responses(
            (status = OK),
            (status = BAD_REQUEST),
            (status = INTERNAL_SERVER_ERROR),
        )
    )
)]
pub async fn add_product(
    State(app): State<AppModule>,
    Path(category_id): Path<CategoryId>,
    Json(req): Json<AddProduct>
) -> Result<StatusCode, StatusCode> {
    let cmd = req.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if let Err(e) = CategoryCommandService::execute(app.category_command_service(), category_id, cmd).await {
        tracing::error!("failed to add product to category: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::OK)
}



#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        delete,
        path = "/categories/{category_id}/{product_id}",
        params(
            ("category_id" = Uuid, Path),
            ("product_id"  = Uuid, Path)
        ),
        responses(
            (status = OK),
            (status = INTERNAL_SERVER_ERROR),
        )
    )
)]
pub async fn remove_product(
    State(app): State<AppModule>,
    Path((category_id, product_id)): Path<(CategoryId, ProductId)>,
) -> Result<StatusCode, StatusCode> {
    let cmd = CategoryCommand::RemoveProduct { id: product_id };
    if let Err(e) = CategoryCommandService::execute(app.category_command_service(), category_id, cmd).await {
        tracing::error!("failed to remove product from category: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::OK)
}



#[cfg_attr(
    feature = "apidoc",
    utoipa::path(
        put,
        path = "/categories/{category_id}",
        params(
            ("category_id" = Uuid, Path),
        ),
        responses(
            (status = OK),
            (status = BAD_REQUEST),
            (status = INTERNAL_SERVER_ERROR),
        )
    )
)]
pub async fn change_product_ordering(
    State(app): State<AppModule>,
    Path(category_id): Path<CategoryId>,
    Json(req): Json<ChangeProductOrdering>
) -> Result<StatusCode, StatusCode> {
    let cmd = req.try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if let Err(e) = CategoryCommandService::execute(app.category_command_service(), category_id, cmd).await {
        tracing::error!("failed to change product ordering: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }
    
    Ok(StatusCode::NO_CONTENT)
}

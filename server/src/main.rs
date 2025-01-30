use axum::extract::DefaultBodyLimit;
use axum::routing::{delete, get};
use axum::Router;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener;
use server::AppModule;
use server::routing::*;
use server::errors::UnrecoverableError;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    let _guard = server::logging::init();
    
    let app = AppModule::setup().await?;

    tracing::info!("starting ez-ticket-api.");

    let categories = Router::new()
        .route("/", get(categories::categories)
            .post(categories::create)
            .put(categories::change_ordering))
        .route("/{category_id}", get(categories::get_products_in_category)
            .post(categories::add_product)
            .put(categories::change_product_ordering)
            .patch(categories::update_name)
            .delete(categories::delete))
        .route("/{category_id}/{product_id}", delete(categories::remove_product));
    
    let products = Router::new()
        .route("/", get(products::get_all_products)
            .post(products::register))
        .route("/{product_id}", get(products::product_details)
            .patch(products::patch)
            .delete(products::delete));
    
    let images = Router::new()
        .route("/{image_id}", get(images::get));
    
    let router = Router::new()
        .nest("/categories", categories)
        .nest("/products", products)
        .nest("/images", images)
        .merge(apidoc())
        .layer(DefaultBodyLimit::disable())
        .with_state(app);

    let listener = TcpListener::bind("0.0.0.0:3650")
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .change_context_lazy(|| UnrecoverableError)?;

    Ok(())
}

#[cfg(feature = "apidoc")]
fn apidoc() -> Router<AppModule> {
    use utoipa::OpenApi;
    
    #[derive(OpenApi)]
    #[openapi(
        paths(
            server::routing::categories::categories,
            server::routing::categories::get_products_in_category,
            server::routing::categories::create,
            server::routing::categories::delete,
            server::routing::categories::update_name,
            server::routing::categories::change_ordering,
            server::routing::categories::add_product,
            server::routing::categories::remove_product,
            server::routing::categories::change_product_ordering,
        
            server::routing::images::get,
        
            server::routing::products::get_all_products,
            server::routing::products::product_details,
            server::routing::products::register,
            server::routing::products::patch,
            server::routing::products::delete
        )
    )]
    struct ApiDocs;
    
    Router::new()
        .merge(utoipa_swagger_ui::SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", ApiDocs::openapi()))
}

#[cfg(not(feature = "apidoc"))]
fn apidoc() -> Router<AppModule> {
    Router::new()
        .route("/docs", get(|| async { "This feature is not enabled." }))
}

async fn shutdown_signal() {
    let user_interrupt = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install keyboard interrupt.")
    };

    tokio::select! {
        _ = user_interrupt => {}
    }
}

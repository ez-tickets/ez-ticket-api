use axum::routing::{delete, get};
use axum::Router;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener;

use server::routing::*;
use server::errors::UnrecoverableError;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    let _guard = server::logging::init();
    
    let app = server::AppModule::setup().await?;

    tracing::info!("starting ez-ticket-api.");

    let categories = Router::new()
        .route("/", get(categories::categories)
            .post(categories::register)
            .put(categories::change_ordering))
        .route("/{category_id}", get(products::get_products_in_category)
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

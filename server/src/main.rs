use axum::routing::get;
use axum::Router;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener;

use server::routing::*;
use server::errors::UnrecoverableError;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    server::logging::init();
    let app = server::AppModule::setup().await?;

    tracing::info!("starting ez-ticket-api.");
    
    let order = Router::new()
        .route("/", get(|| async { "todo" }));

    let category = Router::new()
        .route("/", get(category::categories)
            .post(category::register)
            .patch(category::update_name)
            .delete(category::delete));
    
    let content = Router::new()
        .route("/", get(content::image)
            .patch(content::update));
    
    let catalog = Router::new()
        .route("/", get(|| async { "todo" }));
    
    let router = Router::new()
        .nest("/categories", category)
        .nest("/catalogs", catalog)
        .nest("/contents", content)
        .nest("/orders", order)
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

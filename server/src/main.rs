use axum::Router;
use axum::routing::get;
use error_stack::{Report, ResultExt};
use tokio::net::TcpListener;
use server::errors::UnrecoverableError;

#[tokio::main]
async fn main() -> Result<(), Report<UnrecoverableError>> {
    server::logging::init();
    
    tracing::info!("starting ez-ticket-api.");
    
    let router = Router::new()
        .route("/", get(|| async { "todo" }));
    
    let listener = TcpListener::bind("0.0.0.0:3650").await
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
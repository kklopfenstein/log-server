use axum::{routing::get, Router};
use tracing_subscriber::EnvFilter;
mod handlers;
use crate::handlers::{root_handler, hello_handler, health_handler, not_found_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/health", get(health_handler))
        .fallback(get(not_found_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    let _service = axum::serve(listener, app).await.unwrap();

    tracing::info!("Log server running on port 6969");
}

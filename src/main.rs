use tracing_subscriber::EnvFilter;
use crate::handlers::create_app;

mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    // Load environment variable LOG_FILE_PATH if not specified
    let log_file_path = std::env::var("LOG_FILE_PATH").unwrap_or_else(|_| "/var/log/system.log".to_string());
    
    println!("log_server: Starting on port 6969");
    println!("log_server: Log file: {}", log_file_path);

    let app = create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("log_server: Starting on port 6969");
}
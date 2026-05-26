use std::collections::HashMap;
use tracing_subscriber::EnvFilter;
use crate::config::{Files, LogConfig};
use crate::handlers::create_app;

mod config;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    // Load environment variable CONFIG_FILE, exit with error if not set
    let config_path = std::env::var("CONFIG_FILE").unwrap_or_else(|e| {
        eprintln!("log_server: CONFIG_FILE environment variable is not set. Error: {}", e);
        std::process::exit(1);
    });

    // Load and parse the YAML config file
    let config = LogConfig::load(&config_path).unwrap_or_else(|err| {
        eprintln!("log_server: Failed to load config file '{}'. Error: {}", config_path, err);
        std::process::exit(1);
    });

    // Extract and collect log files from config into HashMap
    let files = config.files.clone().unwrap_or_else(HashMap::new);
    let files: Files = files;
    
    println!("log_server: Starting on port 6969");
    println!("log_server: Config file: {}", config_path);
    println!("log_server: Configured log files: {}", files.len());

    let app = create_app(files).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("log_server: Starting on port 6969");
}
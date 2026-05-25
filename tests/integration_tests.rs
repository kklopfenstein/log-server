use tower_test::client::Client;
use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower::{ServiceExt};

mod handlers {
    use super::*;

    pub fn root_handler() -> Html<&'static str> {
        Html("<!DOCTYPE html>\n<html><head><title>Log Server</title></head><body>\n<h1>Log Server</h1>\n<p>Welcome</p>\n</body></html>")
    }

    pub fn hello_handler() -> impl IntoResponse {
        "Hello, World!"
    }

    pub fn health_handler() -> impl IntoResponse {
        "OK"
    }

    pub fn not_found_handler() -> impl IntoResponse {
        Html("<!DOCTYPE html>\n<html><head><title>404 - Not Found</title></head><body>\n<h1>404 - Not Found</h1>\n</body></html>")
    }
}

#[tokio::test]
async fn test_root_route() {
    let app = Router::new()
        .route("/", handlers::root_handler)
        .route("/hello", get(handlers::hello_handler))
        .route("/health", get(handlers::health_handler))
        .fallback(handlers::not_found_handler);

    let mock = Client::new(app);
    
    let response = mock.get("/").await.expect("failed to execute request");
    assert_eq!(response.status(), 200);
    let text = response.text().await.expect("failed to read body");
    assert!(text.contains("Log Server"));
}

#[tokio::test]
async fn test_hello_route() {
    let app = Router::new()
        .route("/", handlers::root_handler)
        .route("/hello", get(handlers::hello_handler))
        .route("/health", get(handlers::health_handler))
        .fallback(handlers::not_found_handler);

    let mock = Client::new(app);
    
    let response = mock.get("/hello").await.expect("failed to execute request");
    assert_eq!(response.status(), 200);
    let text = response.text().await.expect("failed to read body");
    assert_eq!(text, "Hello, World!");
}

#[tokio::test]
async fn test_health_route() {
    let app = Router::new()
        .route("/", handlers::root_handler)
        .route("/hello", get(handlers::hello_handler))
        .route("/health", get(handlers::health_handler))
        .fallback(handlers::not_found_handler);

    let mock = Client::new(app);
    
    let response = mock.get("/health").await.expect("failed to execute request");
    assert_eq!(response.status(), 200);
    let text = response.text().await.expect("failed to read body");
    assert_eq!(text, "OK");
}

#[tokio::test]
async fn test_404_handler() {
    let app = Router::new()
        .route("/", handlers::root_handler)
        .route("/hello", get(handlers::hello_handler))
        .route("/health", get(handlers::health_handler))
        .fallback(handlers::not_found_handler);

    let mock = Client::new(app);
    
    let response = mock.get("/nonexistent").await.expect("failed to execute request");
    assert_eq!(response.status(), 404);
    let text = response.text().await.expect("failed to read body");
    assert!(text.contains("404"));
}

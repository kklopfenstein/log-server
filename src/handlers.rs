use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

pub async fn root_handler() -> Html<&'static str> {
    Html("<!DOCTYPE html>\n<html><head><title>Log Server</title></head><body>\n<h1>Log Server</h1>\n<p>Welcome</p>\n</body></html>")
}

pub async fn hello_handler() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn health_handler() -> impl IntoResponse {
    "OK"
}

pub async fn not_found_handler() -> Response {
    (
        StatusCode::NOT_FOUND,
        Html("<!DOCTYPE html>\n<html><head><title>404 - Not Found</title></head><body>\n<h1>404 - Not Found</h1>\n</body></html>"),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower::ServiceExt;
    use axum::{
        http::{Method, StatusCode},
        response::{Html, IntoResponse},
        routing::get,
        Router,
    };

    #[tokio::test]
    async fn test_root() {
        let res = create_app()
            .oneshot(axum::http::Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Log Server"));
    }

    #[tokio::test]
    async fn test_hello() {
        let res = create_app()
            .oneshot(axum::http::Request::builder()
                .method(Method::GET)
                .uri("/hello")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, "Hello, World!");
    }

    #[tokio::test]
    async fn test_health() {
        let res = create_app()
            .oneshot(axum::http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, "OK");
    }

    #[tokio::test]
    async fn test_nonexistent() {
        let res = create_app()
            .oneshot(axum::http::Request::builder()
                .method(Method::GET)
                .uri("/nonexistent")
                .body(axum::body::Body::empty())
                .unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
pub fn create_app() -> Router<()> {
    Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/health", get(health_handler))
        .fallback(not_found_handler)
}

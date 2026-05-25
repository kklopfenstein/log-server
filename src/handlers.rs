use axum::{
    response::{Html, IntoResponse},
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

pub async fn not_found_handler() -> impl IntoResponse {
    Html("<!DOCTYPE html>\n<html><head><title>404 - Not Found</title></head><body>\n<h1>404 - Not Found</h1>\n</body></html>")
}

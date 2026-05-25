use axum::{
    Router, extract::{Path, Query}, response::{IntoResponse, Json, Response}, routing::get
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogLine {
    line: String,
    #[serde(rename = "line_num")]
    line_num: u64,
}

#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    #[serde(rename = "cursor")]
    pub cursor: Option<u64>,
    #[serde(rename = "limit")]
    pub limit: Option<u64>,
}

pub async fn create_app() -> Router<()> {
    Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/health", get(health_handler))
        .route("/logs", get(logs_handler))
        .fallback(not_found)
}

pub async fn root_handler() -> impl IntoResponse {
    "<!DOCTYPE html>\n<html><head><title>Log Server</title></head><body>\n<h1>Log Server</h1>\n<p>Welcome</p>\n</body></html>"
}

pub async fn hello_handler() -> &'static str {
    "Hello, World!"
}

pub async fn health_handler() -> &'static str {
    "OK"
}

pub async fn logs_handler(
    path: Path<String>,
    query: Query<LogsQuery>,
) -> Response {
    handle_get_logs(path.0, query.0).await
}

pub async fn handle_get_logs(name: String, query: LogsQuery) -> Response {
    let cursor = query.cursor.unwrap_or(0);
    let limit = query.limit.unwrap_or(100).min(1000).max(1);

    // Try to read the log file
    let file_opt = std::fs::read(&name);
    let file_content = match file_opt {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to read log file: {}", e);
            return not_found().await;
        }
    };
    
    let content = String::from_utf8(file_content)
        .map_err(|e| {
            eprintln!("Failed to decode log file: {}", e);
            StatusCode::NOT_FOUND
        })
        .unwrap_or_default();

    // Split into lines and filter empty lines
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        return (StatusCode::OK, Json(Vec::<LogLine>::new())).into_response();
    }

    // cursor is 1-indexed position to fetch from (0 means skip all from start)
    let start_index = cursor as usize;
    let available = lines.len();
    let final_limit = std::cmp::min(limit as usize, available - start_index);

    if start_index >= available {
        return not_found().await;
    }

    // Build Vec<LogLine>
    let result_lines: Vec<LogLine> = lines[start_index..start_index + final_limit]
        .iter()
        .enumerate()
        .map(|(num, l)| LogLine {
            line: (*l).to_string(),
            line_num: (start_index + num + 1) as u64,
        })
        .collect();

    (StatusCode::OK, Json(result_lines)).into_response()
}

pub async fn not_found() -> Response {
    let body = String::from("<!DOCTYPE html>\n<html><head><title>404 - Not Found</title></head><body>\n<h1>404 - Not Found</h1>\n</body></html>");
    (StatusCode::NOT_FOUND, body).into_response()
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;
    use super::logs_handler;
    use super::not_found;
    use super::root_handler;
    use super::hello_handler;
    use super::health_handler;
    use super::LogsQuery;
    use super::LogLine;
    use super::Query;
    use axum::http::StatusCode;
use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_root_handler() {
        let response = root_handler().await.into_response();
        let (parts, body) = response.into_parts();
        let status = parts.status;
        assert_eq!(status, StatusCode::OK);
        //let body_str = body.into_string().unwrap();
        let body_bytes = axum::body::to_bytes(body, usize::MAX);
        let body_str = String::from_utf8(body_bytes.await.unwrap().to_vec()).unwrap();
        assert!(body_str.contains("<h1>Log Server</h1>"));
        assert!(body_str.contains("<title>Log Server</title>"));
    }

    #[tokio::test]
    async fn test_hello_handler() {
        let response = hello_handler().await;
        assert_eq!(response, "Hello, World!");
    }

    #[tokio::test]
    async fn test_health_handler() {
        let response = health_handler().await;
        assert_eq!(response, "OK");
    }

    #[tokio::test]
    async fn test_logs_handler_with_file() {
        let temp_file = "target/debug/temp_test.log";
        let log_content = "Line 1\nLine 2\nLine 3\n";
        std::fs::write(temp_file, log_content).unwrap();

        let query = LogsQuery {
            cursor: Some(0),
            limit: Some(3),
        };

        let response = logs_handler(Path(temp_file.to_string()), Query(query)).await.into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::OK);
        let body = response.into_body();
        let body_bytes = axum::body::to_bytes(body, usize::MAX);
        let body_str = String::from_utf8(body_bytes.await.unwrap().to_vec()).unwrap();

        let lines: Vec<LogLine> = serde_json::from_str(&body_str).unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].line, "Line 1");
        assert_eq!(lines[0].line_num, 1);
        assert_eq!(lines[2].line_num, 3);

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_logs_handler_with_empty_file() {
        let temp_file = "target/debug/temp_empty.log";
        std::fs::write(temp_file, "").unwrap();

        let query = LogsQuery {
            cursor: None,
            limit: None,
        };

        let response = logs_handler(Path(temp_file.to_string()), Query(query)).await;
        let (_, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, usize::MAX);
        let body_str = String::from_utf8(body_bytes.await.unwrap().to_vec()).unwrap();

        let lines: Vec<LogLine> = serde_json::from_str(&body_str).unwrap();
        assert!(lines.is_empty());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_logs_handler_with_cursor() {
        let temp_file = "target/debug/temp_cursor.log";
        let log_content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n";
        std::fs::write(temp_file, log_content).unwrap();

        let query = LogsQuery {
            cursor: Some(3),
            limit: Some(5),
        };

        let response = logs_handler(Path(temp_file.to_string()), Query(query)).await;
        let (_, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, usize::MAX);
        let body_str = String::from_utf8(body_bytes.await.unwrap().to_vec()).unwrap();

        let lines: Vec<LogLine> = serde_json::from_str(&body_str).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].line_num, 4);
        assert_eq!(lines[1].line_num, 5);

        std::fs::remove_file(temp_file).unwrap();
    }

    #[tokio::test]
    async fn test_not_found() {
        let response = not_found().await;
        let (parts, _) = response.into_parts();
        let status = parts.status;
        assert_eq!(status, StatusCode::NOT_FOUND);
    }
}

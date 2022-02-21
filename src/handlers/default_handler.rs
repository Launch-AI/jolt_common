use axum::{http::StatusCode, response::IntoResponse};

// Default handler
pub async fn default_handler() -> impl IntoResponse {
    (StatusCode::OK, "Default")
}

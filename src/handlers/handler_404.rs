use axum::{body::Body, http::Response};

use crate::errors::not_found::NotFoundError;

// Handler 404 - Not Found
pub async fn handler_404() -> Response<Body> {
    NotFoundError::new("Not Found").into_response()
}

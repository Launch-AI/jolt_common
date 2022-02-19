// Not found Error

use axum::{body::Body, http::Response};

use super::custom_error::CustomError;

pub struct NotFoundError {
    pub message: String,
}

impl NotFoundError {
    pub fn new(error_message: &str) -> Self {
        Self {
            message: error_message.to_string(),
        }
    }

    pub fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&CustomError::new(
                    &404.to_string(),
                    &self.message.to_string(),
                ))
                .unwrap(),
            ))
            .unwrap()
    }
}

use axum::{
    body::Body,
    http::{Response, StatusCode},
};

// Invalid Errpr
// Error code for invalid - 400
pub struct InvalidError {
    pub message: String,
}

impl InvalidError {
    pub fn new(error_message: &str) -> Self {
        Self {
            message: error_message.to_string(),
        }
    }

    // Into Response
    pub fn into_response(self) -> Response<Body> {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(self.message))
            .unwrap()
    }
}

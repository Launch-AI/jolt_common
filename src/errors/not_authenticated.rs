use axum::{http::Response, body::Body};

use super::custom_error::CustomError;



pub struct _NotAuthenticatedError;

impl _NotAuthenticatedError {
    pub fn _into_response(self) -> Response<Body> {
        Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&CustomError::new(&401.to_string(), "Not Authenticated"))
                    .unwrap(),
            ))
            .unwrap()
    }

    pub fn _into(self) -> Result<(), Response<Body>> {
        Err(self._into_response())
    }

    // pub fn new() -> anyhow::Error {
    //     anyhow!("Not Authenticated")
    // }
}

// Not authorized Error

use axum::{http::Response, body::Body};

use super::custom_error::CustomError;




pub struct _NotAuthorizedError;

impl _NotAuthorizedError {
    fn _into_response(self) -> Response<Body> {
        Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&CustomError::new(&401.to_string(), "Not Authorized"))
                    .unwrap(),
            ))
            .unwrap()
    }
}


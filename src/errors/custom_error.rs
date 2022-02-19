use serde::{Deserialize, Serialize};

//
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError {
    pub status_code: String,
    pub message: String,
}

// Extend CustomError with a new method to create a new error

// Status codes for errors
// 404 - Not found
// 500 - Internal server error
// 400 - Bad request
// 401 - Unauthorized
// 403 - Forbidden
impl CustomError {
    pub fn new(status_code: &str, message: &str) -> Self {
        Self {
            status_code: status_code.to_string(),
            message: message.to_string(),
        }
    }

    pub fn _status_code(&self) -> &str {
        &self.status_code
    }

    pub fn _message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {} - {}", self.status_code, self.message)
    }
}

impl std::error::Error for CustomError {
    fn description(&self) -> &str {
        &self.message
    }
}

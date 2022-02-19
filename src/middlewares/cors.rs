// Cors Middleware

use tower_http::cors::{any, CorsLayer};

pub fn cors() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods(any())
        .allow_origin(any())
        .allow_credentials(true)
        .allow_headers(any());
    cors
}

// Cors Middleware

use tower_http::cors::{any, Any, CorsLayer};

pub fn cors() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_credentials(true)
        .allow_headers(Any);
    cors
}

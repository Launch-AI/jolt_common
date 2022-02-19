use axum::http::StatusCode;
use tower::{
    layer::util::{Identity, Stack},
    limit::ConcurrencyLimitLayer,
    BoxError, ServiceBuilder,
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    cors::CorsLayer,
    trace::TraceLayer,
};

use super::cors::cors;

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", err),
        )
    }
}

// Middleware stack
pub fn middleware_stack() -> Stack<
    CorsLayer,
    Stack<
        ConcurrencyLimitLayer,
        Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, Identity>,
    >,
> {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(64))
        .layer(cors())
        .into_inner();
    middleware_stack
}

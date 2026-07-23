use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

pub async fn trace(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    let start = Instant::now();

    let response = next.run(request).await;

    let status = response.status();

    tracing::info!(
        method = %method,
        uri = %uri,
        status = %status,
        latency_ms = start.elapsed().as_millis(),
    );

    response
}

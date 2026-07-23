use axum::{extract::Request, middleware::Next, response::Response};

pub async fn trace(request: Request, next: Next) -> Response {
    tracing::info!(
        method = %request.method(),
        uri = %request.uri(),
        "incoming request",
    );

    next.run(request).await
}

use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

pub async fn request_id(mut request: Request, next: Next) -> Response {
    let request_id = Uuid::now_v7();

    request.extensions_mut().insert(request_id);

    let mut response = next.run(request).await;

    response
        .headers_mut()
        .insert("x-request-id", request_id.to_string().parse().unwrap());

    response
}

use axum::{Json, Router, routing::get};
use serde::Serialize;

use crate::{api::response::ApiResponse, state::AppState};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse::ok(HealthResponse { status: "ok" }))
}

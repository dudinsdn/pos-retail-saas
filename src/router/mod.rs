use axum::{Router, middleware};

use crate::{
    api::handler::health,
    middleware::{request_id, trace},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .layer(middleware::from_fn(trace::trace))
        .layer(middleware::from_fn(request_id::request_id))
        .with_state(state)
}

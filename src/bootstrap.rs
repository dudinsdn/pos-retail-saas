use std::net::SocketAddr;

use tokio::net::TcpListener;

use crate::{config, error::AppError, router, state::AppState, telemetry};

pub async fn run() -> Result<(), AppError> {
    let settings = config::load()?;

    telemetry::init(&settings)?;

    let state = AppState::new(settings);

    let app = router::create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

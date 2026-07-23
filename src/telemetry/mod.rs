use tracing_subscriber::{EnvFilter, fmt};

use crate::{config::Settings, error::AppError};

pub fn init(settings: &Settings) -> Result<(), AppError> {
    let filter = EnvFilter::try_new(&settings.log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    fmt().with_env_filter(filter).with_target(true).init();

    Ok(())
}

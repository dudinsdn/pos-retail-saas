use crate::error::AppError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

impl Settings {
    pub fn load() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}

mod settings;

pub use settings::Settings;

use crate::error::AppError;

pub fn load() -> Result<Settings, AppError> {
    Settings::load()
}

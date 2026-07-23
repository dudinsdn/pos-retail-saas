use std::sync::Arc;

use crate::config::Settings;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Inner>,
}

struct Inner {
    settings: Settings,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            inner: Arc::new(Inner { settings }),
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.inner.settings
    }
}

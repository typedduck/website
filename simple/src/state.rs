use std::sync::Arc;

use config::ConfigError;

use crate::settings::{Settings, Site};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct AppState {
    site: Arc<Site>,
}

impl AppState {
    /// Create a clone of the `Site` struct.
    #[inline]
    #[must_use]
    pub fn site(&self) -> Arc<Site> {
        self.site.clone()
    }
}

impl TryFrom<&Settings> for AppState {
    type Error = ConfigError;

    fn try_from(settings: &Settings) -> Result<Self, Self::Error> {
        Ok(Self {
            site: Arc::new(settings.site.clone()),
        })
    }
}

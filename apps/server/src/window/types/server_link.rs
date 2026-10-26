use crate::server::ServerStatus;
use crate::settings::{SavedSettings, SettingsFileError};
use crate::window::ApplySettings;
use tokio::sync::watch;

pub struct ServerLink<'a> {
    pub(crate) status: watch::Receiver<ServerStatus>,
    pub(crate) apply_settings: ApplySettings<'a>,
}

impl<'a> ServerLink<'a> {
    pub fn new(
        status: watch::Receiver<ServerStatus>,
        apply_settings: impl FnMut(&SavedSettings) -> Result<(), SettingsFileError> + 'a,
    ) -> Self {
        Self {
            status,
            apply_settings: Box::new(apply_settings),
        }
    }
}

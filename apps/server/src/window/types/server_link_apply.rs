use crate::settings::{SavedSettings, SettingsFileError};
use crate::window::ServerLink;

impl ServerLink<'_> {
    pub fn apply(&mut self, settings: &SavedSettings) -> Result<(), SettingsFileError> {
        (self.apply_settings)(settings)
    }
}

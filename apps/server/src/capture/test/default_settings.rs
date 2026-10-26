use crate::settings::{SavedSettings, Settings, settings_of};

pub fn default_settings() -> Settings {
    settings_of(&SavedSettings::default())
}

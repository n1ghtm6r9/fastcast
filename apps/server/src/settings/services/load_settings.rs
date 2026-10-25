use crate::settings::{Settings, SettingsError, settings_from};
use std::env;

pub fn load_settings() -> Result<Settings, SettingsError> {
    settings_from(|variable| env::var(variable).ok())
}

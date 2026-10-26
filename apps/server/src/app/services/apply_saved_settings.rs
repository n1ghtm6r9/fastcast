use crate::journal::JournalLevel;
use crate::server::ServerControl;
use crate::settings::{SavedSettings, SettingsFileError, settings_of, write_saved_settings};
use std::path::Path;

pub fn apply_saved_settings(
    control: &ServerControl,
    level: &JournalLevel,
    path: &Path,
    saved: &SavedSettings,
) -> Result<(), SettingsFileError> {
    control.apply(settings_of(saved));
    level.set(saved.log_level);

    write_saved_settings(path, saved)
}

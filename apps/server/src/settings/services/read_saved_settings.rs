use crate::settings::{SavedSettings, SettingsFileError};
use std::path::Path;
use std::{fs, io};

pub fn read_saved_settings(path: &Path) -> Result<SavedSettings, SettingsFileError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(SavedSettings::default());
        }
        Err(error) => {
            return Err(SettingsFileError::Unreadable {
                path: path.to_owned(),
                error,
            });
        }
    };

    serde_json::from_slice::<SavedSettings>(&bytes)
        .map(SavedSettings::within_limits)
        .map_err(|error| SettingsFileError::Unparsable {
            path: path.to_owned(),
            error,
        })
}

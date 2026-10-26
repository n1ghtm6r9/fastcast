use crate::settings::{SavedSettings, SettingsFileError, temporary_path_beside};
use std::path::Path;
use std::{fs, io};

pub fn write_saved_settings(path: &Path, saved: &SavedSettings) -> Result<(), SettingsFileError> {
    let unwritable = |error: io::Error| SettingsFileError::Unwritable {
        path: path.to_owned(),
        error,
    };
    let json = serde_json::to_vec_pretty(saved).map_err(|error| unwritable(error.into()))?;
    let temporary = temporary_path_beside(path);

    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory).map_err(unwritable)?;
    }
    fs::write(&temporary, json).map_err(unwritable)?;
    fs::rename(&temporary, path).map_err(unwritable)
}

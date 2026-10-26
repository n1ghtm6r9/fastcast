use crate::settings::SettingsFileError;
use crate::window::test::TEST_SETTINGS_FILE;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

pub fn saving_fails() -> Result<(), SettingsFileError> {
    Err(SettingsFileError::Unwritable {
        path: PathBuf::from(TEST_SETTINGS_FILE),
        error: io::Error::from(ErrorKind::PermissionDenied),
    })
}

use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum SettingsFileError {
    Unreadable {
        path: PathBuf,
        error: io::Error,
    },
    Unparsable {
        path: PathBuf,
        error: serde_json::Error,
    },
    Unwritable {
        path: PathBuf,
        error: io::Error,
    },
}

impl fmt::Display for SettingsFileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsFileError::Unreadable { path, error } => write!(
                formatter,
                "the settings file {} cannot be read: {}",
                path.display(),
                error
            ),
            SettingsFileError::Unparsable { path, error } => write!(
                formatter,
                "the settings file {} is damaged: {}",
                path.display(),
                error
            ),
            SettingsFileError::Unwritable { path, error } => write!(
                formatter,
                "the settings file {} cannot be written: {}",
                path.display(),
                error
            ),
        }
    }
}

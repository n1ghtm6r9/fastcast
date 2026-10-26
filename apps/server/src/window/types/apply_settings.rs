use crate::settings::{SavedSettings, SettingsFileError};

pub type ApplySettings<'a> = Box<dyn FnMut(&SavedSettings) -> Result<(), SettingsFileError> + 'a>;

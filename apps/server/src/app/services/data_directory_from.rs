use crate::app::{APP_DATA_VARIABLE, DATA_DIRECTORY_NAME};
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

pub fn data_directory_from(lookup: impl Fn(&str) -> Option<OsString>) -> PathBuf {
    lookup(APP_DATA_VARIABLE)
        .filter(|app_data| !app_data.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join(DATA_DIRECTORY_NAME)
}

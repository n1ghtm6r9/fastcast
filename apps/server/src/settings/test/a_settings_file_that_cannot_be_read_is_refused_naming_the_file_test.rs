use crate::fixtures::TemporaryDirectory;
use crate::settings::{SettingsFileError, read_saved_settings};
use std::fs;

#[test]
fn a_settings_file_that_cannot_be_read_is_refused_naming_the_file() {
    let directory = TemporaryDirectory::new("unreadable-settings-file");
    let path = directory.path().join("settings.json");
    fs::create_dir(&path).expect("a directory takes the file's place");

    let refused = read_saved_settings(&path).expect_err("an unreadable file is refused");

    assert!(matches!(refused, SettingsFileError::Unreadable { .. }));
    assert!(refused.to_string().contains(&path.display().to_string()));
}

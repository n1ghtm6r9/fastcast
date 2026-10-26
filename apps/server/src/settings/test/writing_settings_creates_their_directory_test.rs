use crate::fixtures::TemporaryDirectory;
use crate::settings::{SavedSettings, write_saved_settings};

#[test]
fn writing_settings_creates_their_directory() {
    let directory = TemporaryDirectory::new("settings-directory");
    let path = directory.path().join("fastcast").join("settings.json");

    write_saved_settings(&path, &SavedSettings::default()).expect("the settings are written");

    assert!(path.is_file());
}

use crate::fixtures::TemporaryDirectory;
use crate::settings::{SavedSettings, SettingsFileError, write_saved_settings};

#[test]
fn settings_that_cannot_be_written_are_refused_naming_the_file() {
    let directory = TemporaryDirectory::new("unwritable-settings-file");
    let blocker = directory.write("fastcast", b"a file where the directory should be");
    let path = blocker.join("settings.json");

    let refused = write_saved_settings(&path, &SavedSettings::default())
        .expect_err("a file cannot hold a directory");

    assert!(matches!(refused, SettingsFileError::Unwritable { .. }));
    assert!(refused.to_string().contains(&path.display().to_string()));
}

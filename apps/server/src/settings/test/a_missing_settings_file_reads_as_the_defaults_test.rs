use crate::fixtures::TemporaryDirectory;
use crate::settings::{SavedSettings, read_saved_settings};

#[test]
fn a_missing_settings_file_reads_as_the_defaults() {
    let directory = TemporaryDirectory::new("missing-settings-file");

    let saved = read_saved_settings(&directory.path().join("settings.json"));

    assert_eq!(
        saved.expect("a missing file is not an error"),
        SavedSettings::default()
    );
}

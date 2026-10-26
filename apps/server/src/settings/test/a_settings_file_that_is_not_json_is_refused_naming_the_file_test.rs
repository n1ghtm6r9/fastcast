use crate::fixtures::TemporaryDirectory;
use crate::settings::{SettingsFileError, read_saved_settings};

#[test]
fn a_settings_file_that_is_not_json_is_refused_naming_the_file() {
    let directory = TemporaryDirectory::new("damaged-settings-file");
    let path = directory.write("settings.json", b"{ not json");

    let refused = read_saved_settings(&path).expect_err("a damaged file is refused");

    assert!(matches!(refused, SettingsFileError::Unparsable { .. }));
    assert!(refused.to_string().contains(&path.display().to_string()));
}

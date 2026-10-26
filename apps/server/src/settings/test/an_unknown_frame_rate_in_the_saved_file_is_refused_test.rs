use crate::fixtures::TemporaryDirectory;
use crate::settings::{SettingsFileError, read_saved_settings};

#[test]
fn an_unknown_frame_rate_in_the_saved_file_is_refused() {
    let directory = TemporaryDirectory::new("unknown-frame-rate");
    let path = directory.write("settings.json", br#"{"frame_rate":"75"}"#);

    let refused = read_saved_settings(&path).expect_err("an unknown frame rate is refused");

    assert!(matches!(refused, SettingsFileError::Unparsable { .. }));
}

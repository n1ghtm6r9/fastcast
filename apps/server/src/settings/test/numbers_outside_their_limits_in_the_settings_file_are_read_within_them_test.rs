use crate::fixtures::TemporaryDirectory;
use crate::settings::read_saved_settings;

#[test]
fn numbers_outside_their_limits_in_the_settings_file_are_read_within_them() {
    let directory = TemporaryDirectory::new("settings-beyond-limits");
    let path = directory.write("settings.json", br#"{"capture_fps":1000}"#);

    let saved = read_saved_settings(&path).expect("the settings are read");

    assert_eq!(saved.capture_fps, 240);
}

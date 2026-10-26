use crate::fixtures::TemporaryDirectory;
use crate::settings::{LogLevel, SavedSettings, read_saved_settings, write_saved_settings};

#[test]
fn written_settings_are_read_back_unchanged() {
    let directory = TemporaryDirectory::new("written-settings");
    let path = directory.path().join("settings.json");
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        capture_fps: 120,
        log_level: LogLevel::Debug,
        ..SavedSettings::default()
    };

    write_saved_settings(&path, &saved).expect("the settings are written");

    assert_eq!(
        read_saved_settings(&path).expect("the settings are read"),
        saved
    );
}

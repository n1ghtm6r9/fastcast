use crate::app::apply_saved_settings;
use crate::app::test::{TEST_COMPUTER_NAME, idle_journal_level, start_recording_control};
use crate::fixtures::TemporaryDirectory;
use crate::settings::{LogLevel, SavedSettings, read_saved_settings};

#[tokio::test(flavor = "multi_thread")]
async fn applied_settings_are_written_to_the_settings_file() {
    let directory = TemporaryDirectory::new("applied-settings-are-written");
    let path = directory.path().join("settings.json");
    let (control, _started) = start_recording_control();
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        log_level: LogLevel::Trace,
        ..SavedSettings::default()
    };

    let written = apply_saved_settings(
        &control,
        &idle_journal_level(),
        &path,
        &saved,
        TEST_COMPUTER_NAME,
    );

    assert!(written.is_ok(), "{written:?}");
    assert_eq!(read_saved_settings(&path).expect("the file reads"), saved);
    control.stop().await;
}

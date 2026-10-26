use crate::app::apply_saved_settings;
use crate::app::test::{idle_journal_level, next_settings, start_recording_control};
use crate::fixtures::TemporaryDirectory;
use crate::settings::{SavedSettings, SettingsFileError, settings_of};

#[tokio::test(flavor = "multi_thread")]
async fn settings_that_cannot_be_written_still_reach_the_server() {
    let directory = TemporaryDirectory::new("unwritten-settings-reach-the-server");
    let blocker = directory.write("fastcast", b"a file where the directory should be");
    let (control, mut started) = start_recording_control();
    next_settings(&mut started).await;
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        ..SavedSettings::default()
    };

    let written = apply_saved_settings(
        &control,
        &idle_journal_level(),
        &blocker.join("settings.json"),
        &saved,
    );

    assert!(matches!(written, Err(SettingsFileError::Unwritable { .. })));
    assert_eq!(next_settings(&mut started).await, settings_of(&saved));
    control.stop().await;
}

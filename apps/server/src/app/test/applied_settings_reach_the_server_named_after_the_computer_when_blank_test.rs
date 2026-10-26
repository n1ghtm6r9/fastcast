use crate::app::apply_saved_settings;
use crate::app::test::{
    TEST_COMPUTER_NAME, idle_journal_level, next_settings, start_recording_control,
};
use crate::fixtures::TemporaryDirectory;
use crate::settings::{SavedSettings, settings_of};

#[tokio::test(flavor = "multi_thread")]
async fn applied_settings_reach_the_server_named_after_the_computer_when_blank() {
    let directory = TemporaryDirectory::new("applied-settings-reach-the-server");
    let (control, mut started) = start_recording_control();
    next_settings(&mut started).await;
    let saved = SavedSettings {
        server_name: "  ".to_owned(),
        capture_fps: 120,
        ..SavedSettings::default()
    };

    let _ = apply_saved_settings(
        &control,
        &idle_journal_level(),
        &directory.path().join("settings.json"),
        &saved,
        TEST_COMPUTER_NAME,
    );

    let restarted = next_settings(&mut started).await;
    assert_eq!(restarted, settings_of(&saved, TEST_COMPUTER_NAME));
    assert_eq!(restarted.server_name, TEST_COMPUTER_NAME);
    control.stop().await;
}

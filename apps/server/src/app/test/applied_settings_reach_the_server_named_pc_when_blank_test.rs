use crate::app::apply_saved_settings;
use crate::app::test::{idle_journal_level, next_settings, start_recording_control};
use crate::fixtures::TemporaryDirectory;
use crate::settings::{FrameRate, SavedSettings, settings_of};

#[tokio::test(flavor = "multi_thread")]
async fn applied_settings_reach_the_server_named_pc_when_blank() {
    let directory = TemporaryDirectory::new("applied-settings-reach-the-server");
    let (control, mut started) = start_recording_control();
    next_settings(&mut started).await;
    let saved = SavedSettings {
        server_name: "  ".to_owned(),
        frame_rate: FrameRate::Fps120,
        ..SavedSettings::default()
    };

    let _ = apply_saved_settings(
        &control,
        &idle_journal_level(),
        &directory.path().join("settings.json"),
        &saved,
    );

    let restarted = next_settings(&mut started).await;
    assert_eq!(restarted, settings_of(&saved));
    assert_eq!(restarted.server_name, "PC");
    control.stop().await;
}

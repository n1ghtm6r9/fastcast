use crate::app::apply_saved_settings;
use crate::app::test::start_recording_control;
use crate::fixtures::TemporaryDirectory;
use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::{LogLevel, SavedSettings};
use std::sync::Arc;
use tracing::debug;

#[tokio::test(flavor = "multi_thread")]
async fn applied_settings_change_the_journal_level() {
    let directory = TemporaryDirectory::new("applied-settings-change-the-level");
    let (control, _started) = start_recording_control();
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);
    let saved = SavedSettings {
        log_level: LogLevel::Debug,
        ..SavedSettings::default()
    };

    tracing::subscriber::with_default(subscriber, || {
        let _ = apply_saved_settings(
            &control,
            &level,
            &directory.path().join("settings.json"),
            &saved,
        );
        debug!("a detail");
    });

    let shown = lines.lines();
    assert!(
        shown.iter().any(|line| line.ends_with(" DEBUG a detail")),
        "{shown:?}"
    );
    control.stop().await;
}

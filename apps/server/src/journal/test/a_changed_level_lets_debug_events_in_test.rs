use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::sync::Arc;
use tracing::debug;

#[test]
fn a_changed_level_lets_debug_events_in() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || {
        debug!("hidden");
        level.set(LogLevel::Debug);
        debug!("shown");
    });

    let shown = lines.lines();
    assert_eq!(shown.len(), 1);
    assert!(shown[0].ends_with(" DEBUG shown"), "{}", shown[0]);
}

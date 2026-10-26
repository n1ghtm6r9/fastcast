use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::sync::Arc;
use tracing::info;

#[test]
fn a_message_over_several_lines_fills_several_journal_lines() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || info!("first\r\nsecond"));

    let shown = lines.lines();
    assert_eq!(shown.len(), 2, "{shown:?}");
    assert!(shown[0].ends_with(" INFO first"), "{}", shown[0]);
    assert_eq!(shown[1], "second");
}

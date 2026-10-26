use crate::fixtures::TemporaryDirectory;
use crate::journal::{JournalLines, JournalSink, journal_subscriber, open_journal_file};
use crate::settings::LogLevel;
use std::fs;
use std::sync::Arc;
use tracing::info;

#[test]
fn an_event_reaches_the_journal_lines_and_the_journal_file() {
    let directory = TemporaryDirectory::new("journal-event");
    let path = directory.path().join("fastcast.log");
    let lines = Arc::new(JournalLines::new(10));
    let file = open_journal_file(&path).expect("the journal file opens");
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), Some(file)), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || info!("fastcast is listening"));

    let shown = lines.lines();
    assert_eq!(shown.len(), 1);
    assert!(
        shown[0].ends_with(" INFO fastcast is listening"),
        "{}",
        shown[0]
    );
    let written = fs::read_to_string(&path).expect("the journal file is read");
    assert!(written.contains(" INFO fastcast is listening"), "{written}");
}

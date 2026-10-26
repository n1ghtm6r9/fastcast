use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::sync::Arc;
use tracing::info;

#[test]
fn journal_lines_start_with_the_time_of_day() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || info!("a viewer joined"));

    let line = lines.lines().remove(0);
    let stamp: Vec<char> = line.chars().take(13).collect();
    let shaped = stamp
        .iter()
        .enumerate()
        .all(|(index, character)| match index {
            2 | 5 => *character == ':',
            8 => *character == '.',
            12 => *character == ' ',
            _ => character.is_ascii_digit(),
        });
    assert!(shaped && stamp.len() == 13, "{line}");
}

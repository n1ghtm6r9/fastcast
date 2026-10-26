use crate::journal::{JournalLevel, JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::sync::Arc;

pub fn idle_journal_level() -> JournalLevel {
    let lines = Arc::new(JournalLines::new(10));
    let (_, level) = journal_subscriber(JournalSink::new(lines, None), LogLevel::Info);

    level
}

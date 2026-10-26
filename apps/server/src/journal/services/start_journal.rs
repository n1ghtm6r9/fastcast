use crate::journal::{
    JOURNAL_LINE_LIMIT, Journal, JournalLines, JournalSink, journal_subscriber, open_journal_file,
};
use crate::settings::LogLevel;
use std::path::Path;
use std::sync::Arc;
use tracing::warn;
use tracing_subscriber::util::SubscriberInitExt;

pub fn start_journal(level: LogLevel, file_path: &Path) -> Journal {
    let lines = Arc::new(JournalLines::new(JOURNAL_LINE_LIMIT));
    let (file, problem) = match open_journal_file(file_path) {
        Ok(file) => (Some(file), None),
        Err(problem) => (None, Some(problem)),
    };
    let (subscriber, level) = journal_subscriber(JournalSink::new(lines.clone(), file), level);

    let _ = subscriber.try_init();
    if let Some(problem) = problem {
        warn!(
            "the journal file {} cannot be written: {problem}",
            file_path.display()
        );
    }

    Journal { lines, level }
}

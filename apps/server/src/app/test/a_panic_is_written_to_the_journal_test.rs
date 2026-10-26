use crate::app::log_panics;
use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::panic;
use std::sync::Arc;

#[test]
fn a_panic_is_written_to_the_journal() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || {
        log_panics();
        let _ = panic::catch_unwind(|| panic!("the capture broke"));
    });

    let shown = lines.lines();
    assert!(
        shown
            .iter()
            .any(|line| line.contains(" ERROR fastcast panicked at ")
                && line.ends_with(": the capture broke")),
        "{shown:?}"
    );
}

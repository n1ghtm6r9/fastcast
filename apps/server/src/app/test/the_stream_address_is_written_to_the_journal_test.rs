use crate::app::log_stream_address;
use crate::journal::{JournalLines, JournalSink, journal_subscriber};
use crate::settings::LogLevel;
use std::sync::Arc;

#[test]
fn the_stream_address_is_written_to_the_journal() {
    let lines = Arc::new(JournalLines::new(10));
    let (subscriber, _level) =
        journal_subscriber(JournalSink::new(lines.clone(), None), LogLevel::Info);

    tracing::subscriber::with_default(subscriber, || {
        log_stream_address(
            "192.168.0.10:3278"
                .parse()
                .expect("a literal socket address"),
        );
    });

    let shown = lines.lines();
    assert!(
        shown.iter().any(|line| line.ends_with(
            " INFO FastCast is listening on udp 192.168.0.10:3278, the TV finds it by itself"
        )),
        "{shown:?}"
    );
}

use crate::stream::LinkJournal;
use std::time::{Duration, Instant};
use wire::Feedback;

#[test]
fn five_seconds_without_packets_lose_none() {
    let start = Instant::now();
    let mut journal = LinkJournal::default();
    journal.record(&Feedback::default(), 10, start);

    let line = journal
        .record(&Feedback::default(), 10, start + Duration::from_secs(5))
        .unwrap_or_default();

    assert!(
        line.starts_with("0 of 0 packets arrived (0.0% lost)"),
        "{line}"
    );
}

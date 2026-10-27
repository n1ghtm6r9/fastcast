use crate::stream::LinkJournal;
use std::time::{Duration, Instant};
use wire::Feedback;

#[test]
fn the_link_journal_stays_quiet_for_five_seconds() {
    let start = Instant::now();
    let mut journal = LinkJournal::default();
    let later = Feedback {
        received_packets: 50,
        highest_sequence: 50,
        ..Feedback::default()
    };
    journal.record(&Feedback::default(), 10, start);

    let line = journal.record(&later, 10, start + Duration::from_millis(4999));

    assert_eq!(line, None);
}

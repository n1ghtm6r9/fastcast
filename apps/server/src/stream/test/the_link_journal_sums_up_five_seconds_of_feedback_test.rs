use crate::stream::LinkJournal;
use std::time::{Duration, Instant};
use wire::Feedback;

#[test]
fn the_link_journal_sums_up_five_seconds_of_feedback() {
    let start = Instant::now();
    let mut journal = LinkJournal::default();
    let first = Feedback {
        received_packets: 100,
        highest_sequence: 100,
        lost_frames: 1,
        recovered_shards: 5,
        ..Feedback::default()
    };
    let last = Feedback {
        received_packets: 1090,
        highest_sequence: 1100,
        lost_frames: 3,
        recovered_shards: 25,
        ..Feedback::default()
    };
    journal.record(&first, 10, start);

    let line = journal.record(&last, 12, start + Duration::from_secs(5));

    assert_eq!(
        line.as_deref(),
        Some(
            "990 of 1000 packets arrived (1.0% lost), 2 frames lost, 20 shards rebuilt, 0 keyframes asked, FEC 12%"
        )
    );
}

use crate::stream::LinkJournal;
use std::time::{Duration, Instant};
use wire::Feedback;

#[test]
fn each_link_line_starts_the_next_five_seconds() {
    let start = Instant::now();
    let mut journal = LinkJournal::default();
    let asking = Feedback {
        keyframe_needed: true,
        ..Feedback::default()
    };
    let counted = |received_packets, highest_sequence| Feedback {
        received_packets,
        highest_sequence,
        ..Feedback::default()
    };
    journal.record(&asking, 10, start);
    journal.record(&counted(500, 500), 10, start + Duration::from_secs(5));

    let early = journal.record(&counted(700, 700), 10, start + Duration::from_secs(9));
    let line = journal.record(&counted(900, 1000), 10, start + Duration::from_secs(10));

    assert_eq!(early, None);
    assert_eq!(
        line.as_deref(),
        Some(
            "400 of 500 packets arrived (20.0% lost), 0 frames lost, 0 shards rebuilt, 0 keyframes asked, FEC 10%"
        )
    );
}

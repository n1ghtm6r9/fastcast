use crate::stream::LinkJournal;
use std::time::{Duration, Instant};
use wire::Feedback;

#[test]
fn a_keyframe_ask_repeated_until_it_arrives_counts_once() {
    let start = Instant::now();
    let asking = Feedback {
        keyframe_needed: true,
        ..Feedback::default()
    };
    let mut journal = LinkJournal::default();
    for (millis, feedback) in [
        (0, asking),
        (100, asking),
        (200, Feedback::default()),
        (300, asking),
    ] {
        journal.record(&feedback, 10, start + Duration::from_millis(millis));
    }

    let line = journal
        .record(&Feedback::default(), 10, start + Duration::from_secs(5))
        .unwrap_or_default();

    assert!(line.contains(", 2 keyframes asked,"), "{line}");
}

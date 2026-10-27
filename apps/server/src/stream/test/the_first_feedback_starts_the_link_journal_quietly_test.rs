use crate::stream::LinkJournal;
use std::time::Instant;
use wire::Feedback;

#[test]
fn the_first_feedback_starts_the_link_journal_quietly() {
    let mut journal = LinkJournal::default();

    assert_eq!(
        journal.record(&Feedback::default(), 10, Instant::now()),
        None
    );
}

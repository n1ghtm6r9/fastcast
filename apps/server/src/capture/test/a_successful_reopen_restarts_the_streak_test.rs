use crate::capture::{FailedReopen, REOPEN_GIVE_UP_AFTER, ReopenFailures};
use std::time::Instant;

#[test]
fn a_successful_reopen_restarts_the_streak() {
    let start = Instant::now();
    let later = start + REOPEN_GIVE_UP_AFTER;
    let mut failures = ReopenFailures::default();

    failures.failed(start);
    failures.succeeded();

    assert_eq!(failures.failed(later), FailedReopen::Log);
}

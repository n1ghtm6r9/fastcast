use crate::capture::{FailedReopen, REOPEN_GIVE_UP_AFTER, ReopenFailures};
use std::time::{Duration, Instant};

#[test]
fn reopens_that_keep_failing_give_up_after_the_give_up_time() {
    let start = Instant::now();
    let mut failures = ReopenFailures::default();

    failures.failed(start);

    assert_ne!(
        failures.failed(start + REOPEN_GIVE_UP_AFTER - Duration::from_millis(1)),
        FailedReopen::GiveUp
    );
    assert_eq!(
        failures.failed(start + REOPEN_GIVE_UP_AFTER),
        FailedReopen::GiveUp
    );
}

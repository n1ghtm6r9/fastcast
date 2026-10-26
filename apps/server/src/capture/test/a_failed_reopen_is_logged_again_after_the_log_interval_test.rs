use crate::capture::{FailedReopen, REOPEN_LOG_INTERVAL, ReopenFailures};
use std::time::{Duration, Instant};

#[test]
fn a_failed_reopen_is_logged_again_after_the_log_interval() {
    let start = Instant::now();
    let second = start + REOPEN_LOG_INTERVAL;
    let mut failures = ReopenFailures::default();

    failures.failed(start);

    assert_eq!(failures.failed(second), FailedReopen::Log);
    assert_eq!(
        failures.failed(second + Duration::from_millis(16)),
        FailedReopen::Quiet
    );
}

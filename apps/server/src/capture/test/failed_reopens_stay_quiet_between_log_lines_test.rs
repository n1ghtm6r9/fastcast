use crate::capture::{FailedReopen, REOPEN_LOG_INTERVAL, ReopenFailures};
use std::time::{Duration, Instant};

#[test]
fn failed_reopens_stay_quiet_between_log_lines() {
    let start = Instant::now();
    let mut failures = ReopenFailures::default();

    failures.failed(start);

    assert_eq!(
        failures.failed(start + Duration::from_millis(16)),
        FailedReopen::Quiet
    );
    assert_eq!(
        failures.failed(start + REOPEN_LOG_INTERVAL - Duration::from_millis(1)),
        FailedReopen::Quiet
    );
}

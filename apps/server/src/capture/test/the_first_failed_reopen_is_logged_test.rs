use crate::capture::{FailedReopen, ReopenFailures};
use std::time::Instant;

#[test]
fn the_first_failed_reopen_is_logged() {
    let mut failures = ReopenFailures::default();

    assert_eq!(failures.failed(Instant::now()), FailedReopen::Log);
}

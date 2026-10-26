use crate::capture::{NativeError, REOPEN_GIVE_UP_AFTER, ReopenFailures, note_failed_reopen};
use desktop::DesktopError;
use std::time::Instant;

#[test]
fn a_reopen_that_keeps_failing_ends_the_capture_with_its_last_cause() {
    let start = Instant::now();
    let mut failures = ReopenFailures::default();

    note_failed_reopen(&mut failures, start, DesktopError::AccessLost).expect("the first failure");

    let outcome = note_failed_reopen(
        &mut failures,
        start + REOPEN_GIVE_UP_AFTER,
        DesktopError::AdapterNotFound,
    );

    assert!(matches!(
        outcome,
        Err(NativeError::ReopenGaveUp(DesktopError::AdapterNotFound))
    ));
}

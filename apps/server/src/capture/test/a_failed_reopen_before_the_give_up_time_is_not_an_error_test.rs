use crate::capture::{ReopenFailures, note_failed_reopen};
use desktop::DesktopError;
use std::time::Instant;

#[test]
fn a_failed_reopen_before_the_give_up_time_is_not_an_error() {
    let mut failures = ReopenFailures::default();

    let outcome = note_failed_reopen(&mut failures, Instant::now(), DesktopError::AccessLost);

    assert!(outcome.is_ok());
}

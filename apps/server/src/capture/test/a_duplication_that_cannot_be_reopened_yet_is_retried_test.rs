use crate::capture::{ReopenStep, reopen_step_of};

#[test]
fn a_duplication_that_cannot_be_reopened_yet_is_retried() {
    assert_eq!(reopen_step_of((3840, 2160), None), ReopenStep::Retry);
}

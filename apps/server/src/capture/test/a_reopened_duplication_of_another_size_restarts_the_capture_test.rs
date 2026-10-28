use crate::capture::{ReopenStep, reopen_step_of};

#[test]
fn a_reopened_duplication_of_another_size_restarts_the_capture() {
    assert_eq!(
        reopen_step_of((3840, 2160), Ok((1920, 1080))),
        ReopenStep::Restart
    );
}

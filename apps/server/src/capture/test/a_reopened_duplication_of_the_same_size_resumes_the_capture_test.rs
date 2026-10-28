use crate::capture::{ReopenStep, reopen_step_of};

#[test]
fn a_reopened_duplication_of_the_same_size_resumes_the_capture() {
    assert_eq!(
        reopen_step_of((3840, 2160), Ok((3840, 2160))),
        ReopenStep::Resume
    );
}

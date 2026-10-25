use crate::capture::{CaptureReport, ended_before_the_first_frame};

#[test]
fn a_run_without_frames_ended_before_the_first_frame() {
    assert!(ended_before_the_first_frame(&Ok(CaptureReport {
        frames: 0
    })));
}

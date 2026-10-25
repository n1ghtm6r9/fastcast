use crate::capture::{CaptureReport, ended_before_the_first_frame};

#[test]
fn a_run_that_produced_frames_did_not_end_before_the_first_frame() {
    assert!(!ended_before_the_first_frame(&Ok(CaptureReport {
        frames: 1
    })));
    assert!(!ended_before_the_first_frame(&Ok(CaptureReport {
        frames: 5_000
    })));
}

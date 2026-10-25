use crate::capture::{CaptureReport, encoder_failed};

#[test]
fn a_run_that_ended_before_any_frame_counts_as_an_encoder_failure() {
    assert!(encoder_failed(&Ok(CaptureReport { frames: 0 })));
}

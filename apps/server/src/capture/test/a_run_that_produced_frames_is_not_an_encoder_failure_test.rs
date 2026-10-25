use crate::capture::{CaptureReport, encoder_failed};

#[test]
fn a_run_that_produced_frames_is_not_an_encoder_failure() {
    assert!(!encoder_failed(&Ok(CaptureReport { frames: 1 })));
    assert!(!encoder_failed(&Ok(CaptureReport { frames: 5_000 })));
}

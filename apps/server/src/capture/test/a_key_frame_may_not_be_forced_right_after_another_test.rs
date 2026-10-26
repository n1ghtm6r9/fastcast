use crate::capture::{KeyFrameGate, MIN_KEY_FRAME_INTERVAL};
use std::time::{Duration, Instant};

#[test]
fn a_key_frame_may_not_be_forced_right_after_another() {
    let start = Instant::now();
    let mut gate = KeyFrameGate::default();

    gate.forced(start);

    assert!(!gate.may_force(start));
    assert!(!gate.may_force(start + MIN_KEY_FRAME_INTERVAL - Duration::from_millis(1)));
}

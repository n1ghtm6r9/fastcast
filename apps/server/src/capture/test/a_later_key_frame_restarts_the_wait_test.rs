use crate::capture::{KeyFrameGate, MIN_KEY_FRAME_INTERVAL};
use std::time::{Duration, Instant};

#[test]
fn a_later_key_frame_restarts_the_wait() {
    let start = Instant::now();
    let later = start + MIN_KEY_FRAME_INTERVAL * 3 / 2;
    let mut gate = KeyFrameGate::default();

    gate.forced(start);
    gate.forced(later);

    assert!(!gate.may_force(later + MIN_KEY_FRAME_INTERVAL - Duration::from_millis(1)));
    assert!(gate.may_force(later + MIN_KEY_FRAME_INTERVAL));
}

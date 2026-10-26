use crate::capture::{KeyFrameGate, MIN_KEY_FRAME_INTERVAL};
use std::time::Instant;

#[test]
fn a_key_frame_may_be_forced_once_the_minimum_interval_has_passed() {
    let start = Instant::now();
    let mut gate = KeyFrameGate::default();

    gate.forced(start);

    assert!(gate.may_force(start + MIN_KEY_FRAME_INTERVAL));
    assert!(gate.may_force(start + MIN_KEY_FRAME_INTERVAL * 2));
}

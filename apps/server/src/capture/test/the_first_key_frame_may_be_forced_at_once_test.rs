use crate::capture::KeyFrameGate;
use std::time::Instant;

#[test]
fn the_first_key_frame_may_be_forced_at_once() {
    let gate = KeyFrameGate::default();

    assert!(gate.may_force(Instant::now()));
}

use crate::capture::{KeyFrameGate, key_frame_wanted};
use crate::hub::Hub;
use std::time::Instant;

#[test]
fn a_key_frame_request_is_taken_once_the_gate_is_open() {
    let gate = KeyFrameGate::default();
    let hub = Hub::new();

    hub.request_key_frame();

    assert!(key_frame_wanted(&gate, &hub, Instant::now()));
    assert!(!hub.take_key_frame_request());
}

use crate::capture::{KeyFrameGate, MIN_KEY_FRAME_INTERVAL, key_frame_wanted};
use crate::hub::Hub;
use std::time::Instant;

#[test]
fn a_key_frame_request_stays_in_the_hub_while_the_gate_is_closed() {
    let start = Instant::now();
    let mut gate = KeyFrameGate::default();
    let hub = Hub::new();

    gate.forced(start);
    hub.request_key_frame();

    assert!(!key_frame_wanted(&gate, &hub, start));
    assert!(key_frame_wanted(
        &gate,
        &hub,
        start + MIN_KEY_FRAME_INTERVAL
    ));
}

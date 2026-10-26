use crate::capture::{KeyFrameGate, key_frame_wanted};
use crate::hub::Hub;
use std::time::Instant;

#[test]
fn no_key_frame_is_wanted_without_a_request() {
    let gate = KeyFrameGate::default();
    let hub = Hub::new();

    assert!(!key_frame_wanted(&gate, &hub, Instant::now()));
}

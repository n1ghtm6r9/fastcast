use crate::capture::KeyFrameGate;
use crate::hub::Hub;
use std::time::Instant;

pub fn key_frame_wanted(gate: &KeyFrameGate, hub: &Hub, now: Instant) -> bool {
    gate.may_force(now) && hub.take_key_frame_request()
}

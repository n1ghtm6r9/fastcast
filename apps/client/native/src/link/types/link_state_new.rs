use crate::link::LinkState;
use std::time::Instant;
use wire::{FrameReceiver, ReceiveCounters};

impl LinkState {
    pub fn new(now: Instant) -> LinkState {
        LinkState {
            session: None,
            receiver: FrameReceiver::new(),
            counters: ReceiveCounters::default(),
            next_hello: now,
            next_feedback: now,
            reported_losses: 0,
        }
    }
}

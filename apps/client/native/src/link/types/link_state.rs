use std::time::Instant;
use wire::{FrameReceiver, ReceiveCounters};

#[derive(Debug)]
pub struct LinkState {
    pub session: Option<u32>,
    pub receiver: FrameReceiver,
    pub counters: ReceiveCounters,
    pub next_hello: Instant,
    pub next_feedback: Instant,
    pub reported_losses: u32,
}

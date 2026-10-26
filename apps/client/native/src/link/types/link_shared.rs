use crate::link::EventQueue;
use std::sync::atomic::{AtomicBool, AtomicU32};

#[derive(Debug, Default)]
pub struct LinkShared {
    pub queue: EventQueue,
    pub session: AtomicU32,
    pub next_sequence: AtomicU32,
    pub stop: AtomicBool,
    pub key_frame_requested: AtomicBool,
}

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32};
use tokio::sync::Notify;
use wire::BASE_FEC_PERCENT;

pub struct SessionControl {
    pub(crate) stop: AtomicBool,
    pub(crate) welcome_wanted: AtomicBool,
    pub(crate) key_frame_needed: AtomicBool,
    pub(crate) peer_highest_frame: AtomicU32,
    pub(crate) fec_percent: AtomicU8,
    pub(crate) wake: Notify,
}

impl SessionControl {
    pub fn new() -> Self {
        Self {
            stop: AtomicBool::new(false),
            welcome_wanted: AtomicBool::new(false),
            key_frame_needed: AtomicBool::new(false),
            peer_highest_frame: AtomicU32::new(0),
            fec_percent: AtomicU8::new(BASE_FEC_PERCENT),
            wake: Notify::new(),
        }
    }
}

impl Default for SessionControl {
    fn default() -> Self {
        Self::new()
    }
}

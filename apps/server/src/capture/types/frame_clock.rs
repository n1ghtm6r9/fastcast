use std::time::{Duration, Instant};

pub struct FrameClock {
    pub(crate) interval: Duration,
    pub(crate) next: Option<Instant>,
    pub(crate) next_picture: Option<Instant>,
}

impl FrameClock {
    pub fn new(fps: u32) -> Self {
        Self {
            interval: Duration::from_secs(1) / fps,
            next: None,
            next_picture: None,
        }
    }
}

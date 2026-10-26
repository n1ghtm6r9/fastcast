use crate::capture::FrameClock;
use std::time::{Duration, Instant};

impl FrameClock {
    pub fn until_repeat(&self, now: Instant) -> Duration {
        match self.next {
            None => self.interval,
            Some(next) => next.saturating_duration_since(now),
        }
    }
}

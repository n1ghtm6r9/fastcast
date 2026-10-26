use crate::capture::{FrameClock, INTERVAL_SLACK_DIVISOR};
use std::time::Instant;

impl FrameClock {
    pub fn may_encode(&self, now: Instant) -> bool {
        match self.next {
            None => true,
            Some(next) => now + self.interval / INTERVAL_SLACK_DIVISOR >= next,
        }
    }
}

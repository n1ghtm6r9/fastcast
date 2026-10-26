use crate::capture::{FrameClock, INTERVAL_SLACK_DIVISOR};
use std::time::{Duration, Instant};

impl FrameClock {
    pub fn until_repeat(&self, now: Instant) -> Duration {
        match self.next {
            None => self.interval,
            Some(next) => {
                (next + self.interval / INTERVAL_SLACK_DIVISOR).saturating_duration_since(now)
            }
        }
    }
}

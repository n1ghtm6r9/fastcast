use crate::capture::FrameClock;
use std::time::Instant;

impl FrameClock {
    pub fn encoded_picture(&mut self, arrived: Instant, at: Instant) {
        let following = arrived + self.interval;
        let next = if at > following {
            at + self.interval
        } else {
            following
        };

        self.next = Some(next);
        self.next_picture = Some(next);
    }
}

use crate::capture::FrameClock;
use std::time::Instant;

impl FrameClock {
    pub fn encoded(&mut self, at: Instant) {
        let due = self.next.unwrap_or(at);
        let following = due + self.interval;

        self.next = Some(if at > following {
            at + self.interval
        } else {
            following
        });
    }
}

use crate::capture::FrameClock;
use std::time::Instant;

impl FrameClock {
    pub fn encoded(&mut self, at: Instant) {
        let turn = self.next.map_or(at, |next| next.max(at));

        self.next = Some(turn + self.interval);
    }
}

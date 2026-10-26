use crate::capture::KeyFrameGate;
use std::time::Instant;

impl KeyFrameGate {
    pub fn forced(&mut self, at: Instant) {
        self.last = Some(at);
    }
}

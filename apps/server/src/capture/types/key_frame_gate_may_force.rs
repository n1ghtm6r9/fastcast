use crate::capture::{KeyFrameGate, MIN_KEY_FRAME_INTERVAL};
use std::time::Instant;

impl KeyFrameGate {
    pub fn may_force(&self, now: Instant) -> bool {
        self.last
            .is_none_or(|last| now.saturating_duration_since(last) >= MIN_KEY_FRAME_INTERVAL)
    }
}

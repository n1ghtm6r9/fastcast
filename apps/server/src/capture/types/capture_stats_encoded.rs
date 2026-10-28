use crate::capture::CaptureStats;
use std::time::{Duration, Instant};

impl CaptureStats {
    pub fn encoded(&mut self, at: Instant, took: Duration, picture: bool) {
        if picture {
            self.pictures += 1;
        } else {
            self.repeats += 1;
        }

        if let Some(previous) = self.previous {
            self.longest_pause = self
                .longest_pause
                .max(at.saturating_duration_since(previous));
        }

        self.previous = Some(at);
        self.encoding.push(took);
    }
}

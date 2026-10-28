use crate::capture::{CAPTURE_REPORT_INTERVAL, CaptureStats, duration_spread};
use crate::stream::milliseconds_of;
use std::time::{Duration, Instant};

impl CaptureStats {
    pub fn line(&mut self, now: Instant) -> Option<String> {
        let elapsed = now.saturating_duration_since(self.since);

        if elapsed < CAPTURE_REPORT_INTERVAL {
            return None;
        }

        let seconds = elapsed.as_secs_f64();
        let frames = f64::from(self.pictures + self.repeats) / seconds;
        let pictures = f64::from(self.pictures) / seconds;
        let line = format!(
            "capture: {frames:.1} frames/s, {pictures:.1} of them new pictures, {} pictures replaced before encoding; encoding took {}; waits overran by {}; the longest pause between frames was {:.1} ms",
            self.replaced,
            duration_spread(&mut self.encoding),
            duration_spread(&mut self.overruns),
            milliseconds_of(self.longest_pause)
        );

        self.since = now;
        self.pictures = 0;
        self.repeats = 0;
        self.replaced = 0;
        self.encoding.clear();
        self.overruns.clear();
        self.longest_pause = Duration::ZERO;

        Some(line)
    }
}

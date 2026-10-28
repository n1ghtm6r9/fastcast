use crate::capture::CaptureStats;
use std::time::Duration;

impl CaptureStats {
    pub fn waited(&mut self, overrun: Duration) {
        self.overruns.push(overrun);
    }
}

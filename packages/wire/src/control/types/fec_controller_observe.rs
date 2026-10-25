use crate::control::{BASE_FEC_PERCENT, FEC_DECAY_INTERVAL, FecController, fec_target};
use std::time::Instant;

impl FecController {
    pub fn observe(&mut self, loss: f64, now: Instant) {
        let target = fec_target(loss);

        if target > self.percent {
            self.percent = target;
            self.last_step = now;

            return;
        }

        let elapsed = now.saturating_duration_since(self.last_step);
        let intervals =
            u32::try_from(elapsed.as_nanos() / FEC_DECAY_INTERVAL.as_nanos()).unwrap_or(u32::MAX);
        let steps = u8::try_from(intervals).unwrap_or(u8::MAX);

        self.percent = self.percent.saturating_sub(steps).max(BASE_FEC_PERCENT);
        self.last_step += FEC_DECAY_INTERVAL * intervals;
    }
}

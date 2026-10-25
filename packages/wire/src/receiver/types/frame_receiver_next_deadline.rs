use crate::receiver::FrameReceiver;
use std::time::Instant;

impl FrameReceiver {
    pub fn next_deadline(&self) -> Option<Instant> {
        self.loss_after(self.first_gap()?)
            .map(|deadline| deadline.at)
    }
}

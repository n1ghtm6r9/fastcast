use crate::receiver::{FrameReceiver, ReceivedFrame};
use std::time::Instant;

impl FrameReceiver {
    pub fn poll(&mut self, now: Instant) -> Option<ReceivedFrame> {
        loop {
            if let Some(frame) = self.take_complete_front() {
                if let Some(received) = self.release(frame) {
                    return Some(received);
                }

                continue;
            }

            let deadline = self.loss_after(self.expected)?;

            if deadline.at > now {
                return None;
            }

            self.declare_lost(deadline.end);
        }
    }
}

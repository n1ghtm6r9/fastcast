use crate::support::{POLL_STEP, TestViewer, receive_within};
use std::time::{Duration, Instant};
use wire::ReceivedFrame;

impl TestViewer {
    pub fn next_frame(&mut self, timeout: Duration) -> Option<ReceivedFrame> {
        let deadline = Instant::now() + timeout;

        loop {
            if let Some(frame) = self.receiver.poll(Instant::now()) {
                return Some(frame);
            }

            let remaining = deadline.saturating_duration_since(Instant::now());

            if remaining.is_zero() {
                return None;
            }

            if let Some((bytes, _)) = receive_within(&self.socket, remaining.min(POLL_STEP)) {
                self.take(&bytes);
            }
        }
    }
}

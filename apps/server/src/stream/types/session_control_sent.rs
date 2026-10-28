use crate::stream::{SENT_DELAY_CAPACITY, SessionControl};
use std::time::Duration;

impl SessionControl {
    pub fn sent(&self, delay: Duration) {
        let mut delays = self.lock_sent();

        if delays.len() < SENT_DELAY_CAPACITY {
            delays.push(delay);
        }
    }
}

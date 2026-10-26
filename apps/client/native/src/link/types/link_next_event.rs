use crate::link::{Link, LinkEvent};
use std::time::Duration;

impl Link {
    pub fn next_event(&self, timeout: Duration) -> Option<LinkEvent> {
        self.shared.queue.next(timeout)
    }
}

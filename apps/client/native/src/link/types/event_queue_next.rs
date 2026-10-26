use crate::link::{EventQueue, LinkEvent};
use std::sync::PoisonError;
use std::time::Duration;

impl EventQueue {
    pub fn next(&self, timeout: Duration) -> Option<LinkEvent> {
        let (mut state, _) = self
            .ready
            .wait_timeout_while(self.lock(), timeout, |state| {
                state.events.is_empty() && !state.closed
            })
            .unwrap_or_else(PoisonError::into_inner);
        let event = state.events.pop_front()?;

        if event.is_frame() {
            state.frames -= 1;
        }

        Some(event)
    }
}

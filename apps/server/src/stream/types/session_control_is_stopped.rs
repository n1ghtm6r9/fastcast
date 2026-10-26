use crate::stream::SessionControl;
use std::sync::atomic::Ordering;

impl SessionControl {
    pub fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

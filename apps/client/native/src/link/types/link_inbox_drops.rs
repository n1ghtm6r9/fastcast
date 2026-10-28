use crate::link::Link;
use std::sync::atomic::Ordering;

impl Link {
    pub fn inbox_drops(&self) -> u32 {
        self.shared.inbox_drops.load(Ordering::Relaxed)
    }
}

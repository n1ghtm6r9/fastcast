use crate::link::LinkShared;
use std::sync::atomic::Ordering;

impl LinkShared {
    pub fn take_longest_pause_ms(&self) -> u32 {
        self.longest_pause_ms.swap(0, Ordering::Relaxed)
    }
}

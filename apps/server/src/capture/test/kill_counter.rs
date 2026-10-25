use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct KillCounter {
    kills: Arc<AtomicUsize>,
    armed: bool,
}

impl KillCounter {
    pub fn armed(kills: Arc<AtomicUsize>) -> Self {
        Self { kills, armed: true }
    }

    pub fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for KillCounter {
    fn drop(&mut self) {
        if self.armed {
            self.kills.fetch_add(1, Ordering::SeqCst);
        }
    }
}

use super::FakeBackend;
use std::sync::atomic::Ordering;

impl FakeBackend {
    pub fn kills(&self) -> usize {
        self.killed.load(Ordering::SeqCst)
    }
}

use super::FakeBackend;
use std::sync::atomic::Ordering;

impl FakeBackend {
    pub fn starts(&self) -> usize {
        self.started.load(Ordering::SeqCst)
    }
}

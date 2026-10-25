use super::FakeBackend;
use std::sync::atomic::Ordering;

impl FakeBackend {
    pub fn prepares(&self) -> usize {
        self.prepared.load(Ordering::SeqCst)
    }
}

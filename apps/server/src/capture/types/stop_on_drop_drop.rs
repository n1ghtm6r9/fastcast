use crate::capture::StopOnDrop;
use std::sync::atomic::Ordering;

impl Drop for StopOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

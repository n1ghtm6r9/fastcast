use crate::capture::NativeCapture;
use std::sync::atomic::Ordering;

impl NativeCapture<'_> {
    pub(crate) fn stopped(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

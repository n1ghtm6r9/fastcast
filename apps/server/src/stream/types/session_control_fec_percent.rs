use crate::stream::SessionControl;
use std::sync::atomic::Ordering;

impl SessionControl {
    pub fn fec_percent(&self) -> u8 {
        self.fec_percent.load(Ordering::Acquire)
    }
}

use crate::hub::Hub;
use std::sync::atomic::Ordering;

impl Hub {
    pub fn take_key_frame_request(&self) -> bool {
        self.key_frame_wanted.swap(false, Ordering::AcqRel)
    }
}

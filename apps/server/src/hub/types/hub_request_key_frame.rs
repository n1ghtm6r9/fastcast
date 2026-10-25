use crate::hub::Hub;
use std::sync::atomic::Ordering;

impl Hub {
    pub fn request_key_frame(&self) {
        self.key_frame_wanted.store(true, Ordering::Release);
    }
}

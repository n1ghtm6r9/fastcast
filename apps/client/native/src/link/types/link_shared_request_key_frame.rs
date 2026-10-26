use crate::link::LinkShared;
use std::sync::atomic::Ordering;

impl LinkShared {
    pub fn request_key_frame(&self) {
        self.key_frame_requested.store(true, Ordering::Release);
    }
}

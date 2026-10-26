use crate::stream::SessionControl;
use std::sync::atomic::Ordering;

impl SessionControl {
    pub fn take_key_frame_need(&self, last_key_frame_sent: Option<u32>) -> bool {
        let needed = self.key_frame_needed.swap(false, Ordering::AcqRel);

        needed
            && last_key_frame_sent
                .is_none_or(|last| self.peer_highest_frame.load(Ordering::Acquire) >= last)
    }
}

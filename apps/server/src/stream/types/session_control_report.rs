use crate::stream::SessionControl;
use std::sync::atomic::Ordering;
use wire::Feedback;

impl SessionControl {
    pub fn report(&self, feedback: &Feedback, fec_percent: u8) {
        self.peer_highest_frame
            .store(feedback.highest_frame, Ordering::Release);
        self.fec_percent.store(fec_percent, Ordering::Release);

        if feedback.keyframe_needed {
            self.key_frame_needed.store(true, Ordering::Release);
            self.wake.notify_one();
        }
    }
}

use crate::receiver::FrameReceiver;

impl FrameReceiver {
    pub fn keyframe_needed(&self) -> bool {
        self.waiting_for_keyframe && self.video_lost_while_waiting
    }
}

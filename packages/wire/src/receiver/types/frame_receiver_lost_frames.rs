use crate::receiver::FrameReceiver;

impl FrameReceiver {
    pub fn lost_frames(&self) -> u32 {
        self.lost_frames
    }
}

use crate::receiver::FrameReceiver;

impl FrameReceiver {
    pub fn highest_frame(&self) -> u32 {
        self.highest
    }
}

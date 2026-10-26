use crate::receiver::FrameReceiver;

impl FrameReceiver {
    pub fn highest_frame(&self) -> u32 {
        u32::try_from(self.expected.saturating_sub(1)).unwrap_or(u32::MAX)
    }
}

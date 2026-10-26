use crate::stream::SessionSender;

impl SessionSender {
    pub fn last_key_frame(&self) -> Option<u32> {
        self.last_key_frame
    }
}

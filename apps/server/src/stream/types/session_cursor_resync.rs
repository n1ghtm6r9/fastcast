use crate::stream::SessionCursor;

impl SessionCursor {
    pub fn resync(&mut self) {
        self.waiting_for_key_frame = true;
        self.sent_config = None;
    }
}

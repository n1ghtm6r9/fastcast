use crate::stream::SessionCursor;

impl SessionCursor {
    pub fn lagged(&mut self) {
        self.waiting_for_key_frame = true;
    }
}

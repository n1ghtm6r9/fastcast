use crate::stream::SessionCursor;

impl SessionCursor {
    pub fn is_waiting(&self) -> bool {
        self.waiting_for_key_frame
    }
}

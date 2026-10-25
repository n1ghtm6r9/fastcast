use crate::web::ViewerCursor;

impl ViewerCursor {
    pub fn lagged(&mut self) {
        self.waiting_for_key_frame = true;
    }
}

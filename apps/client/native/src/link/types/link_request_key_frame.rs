use crate::link::Link;

impl Link {
    pub fn request_key_frame(&self) {
        self.shared.request_key_frame();
    }
}

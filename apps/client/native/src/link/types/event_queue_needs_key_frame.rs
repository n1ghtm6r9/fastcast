use crate::link::EventQueue;

impl EventQueue {
    pub fn needs_key_frame(&self) -> bool {
        self.lock().waiting_for_key_frame
    }
}

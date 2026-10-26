use crate::link::EventQueue;

impl EventQueue {
    pub fn is_closed(&self) -> bool {
        self.lock().closed
    }
}

use crate::link::EventQueue;

impl EventQueue {
    pub fn close(&self) {
        self.lock().closed = true;
        self.ready.notify_all();
    }
}

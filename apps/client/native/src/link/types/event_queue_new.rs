use crate::link::{EventQueue, QueueState};
use std::sync::{Condvar, Mutex};

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            state: Mutex::new(QueueState::default()),
            ready: Condvar::new(),
        }
    }
}

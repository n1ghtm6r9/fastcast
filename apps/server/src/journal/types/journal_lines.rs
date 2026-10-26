use std::collections::VecDeque;
use std::sync::Mutex;

pub struct JournalLines {
    pub(crate) lines: Mutex<VecDeque<String>>,
    pub(crate) capacity: usize,
}

impl JournalLines {
    pub fn new(capacity: usize) -> Self {
        Self {
            lines: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }
}

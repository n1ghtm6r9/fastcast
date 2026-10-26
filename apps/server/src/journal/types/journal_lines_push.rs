use crate::journal::JournalLines;
use std::sync::PoisonError;

impl JournalLines {
    pub fn push(&self, line: String) {
        let mut lines = self.lines.lock().unwrap_or_else(PoisonError::into_inner);

        lines.push_back(line);
        while lines.len() > self.capacity {
            lines.pop_front();
        }
    }
}

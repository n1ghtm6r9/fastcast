use crate::journal::JournalLines;
use std::sync::PoisonError;

impl JournalLines {
    pub fn lines(&self) -> Vec<String> {
        self.lines
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .cloned()
            .collect()
    }
}

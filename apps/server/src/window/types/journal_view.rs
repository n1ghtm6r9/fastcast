use crate::journal::JournalLines;
use std::path::PathBuf;
use std::sync::Arc;

pub struct JournalView {
    pub(crate) lines: Arc<JournalLines>,
    pub(crate) file: PathBuf,
}

impl JournalView {
    pub fn new(lines: Arc<JournalLines>, file: PathBuf) -> Self {
        Self { lines, file }
    }
}

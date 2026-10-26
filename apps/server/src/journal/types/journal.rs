use crate::journal::{JournalLevel, JournalLines};
use std::sync::Arc;

#[derive(Clone)]
pub struct Journal {
    pub lines: Arc<JournalLines>,
    pub level: JournalLevel,
}

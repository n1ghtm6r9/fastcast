use crate::journal::{JournalLevel, journal_targets};
use crate::settings::LogLevel;

impl JournalLevel {
    pub fn set(&self, level: LogLevel) {
        let _ = self.handle.reload(journal_targets(level));
    }
}

use crate::stream::{DIAGNOSE_NOTICE_INTERVAL, DiagnoseRepeat};
use std::time::Instant;

impl DiagnoseRepeat {
    pub fn due(&mut self, now: Instant) -> Option<u64> {
        let run = self.run?;
        let waited = self
            .sent
            .is_none_or(|sent| now.saturating_duration_since(sent) >= DIAGNOSE_NOTICE_INTERVAL);

        if !waited {
            return None;
        }

        self.sent = Some(now);

        Some(run)
    }
}

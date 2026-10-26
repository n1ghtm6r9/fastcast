use crate::capture::{FailedReopen, REOPEN_GIVE_UP_AFTER, REOPEN_LOG_INTERVAL, ReopenFailures};
use std::time::Instant;

impl ReopenFailures {
    pub fn failed(&mut self, now: Instant) -> FailedReopen {
        let since = *self.since.get_or_insert(now);

        if now.saturating_duration_since(since) >= REOPEN_GIVE_UP_AFTER {
            return FailedReopen::GiveUp;
        }

        let due = self
            .logged
            .is_none_or(|logged| now.saturating_duration_since(logged) >= REOPEN_LOG_INTERVAL);

        if !due {
            return FailedReopen::Quiet;
        }

        self.logged = Some(now);

        FailedReopen::Log
    }
}

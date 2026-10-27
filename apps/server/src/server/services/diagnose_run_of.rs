use crate::settings::Settings;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn diagnose_run_of(settings: &Settings, now: SystemTime) -> Option<u64> {
    settings.debug.then(|| {
        now.duration_since(UNIX_EPOCH)
            .ok()
            .and_then(|since| u64::try_from(since.as_millis()).ok())
            .unwrap_or_default()
            .max(1)
    })
}

use crate::web::{VIEWER_PING_INTERVAL, VIEWER_SILENCE_TIMEOUT, VIEWER_WRITE_TIMEOUT};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViewerTimings {
    pub ping_interval: Duration,
    pub silence_timeout: Duration,
    pub write_timeout: Duration,
}

impl Default for ViewerTimings {
    fn default() -> Self {
        Self {
            ping_interval: VIEWER_PING_INTERVAL,
            silence_timeout: VIEWER_SILENCE_TIMEOUT,
            write_timeout: VIEWER_WRITE_TIMEOUT,
        }
    }
}

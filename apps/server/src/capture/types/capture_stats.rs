use std::time::{Duration, Instant};

pub struct CaptureStats {
    pub(crate) since: Instant,
    pub(crate) pictures: u32,
    pub(crate) repeats: u32,
    pub(crate) encoding: Vec<Duration>,
    pub(crate) overruns: Vec<Duration>,
    pub(crate) previous: Option<Instant>,
    pub(crate) longest_pause: Duration,
    pub(crate) replaced: u32,
}

impl CaptureStats {
    pub fn new(now: Instant) -> Self {
        Self {
            since: now,
            pictures: 0,
            repeats: 0,
            encoding: Vec::new(),
            overruns: Vec::new(),
            previous: None,
            longest_pause: Duration::ZERO,
            replaced: 0,
        }
    }
}

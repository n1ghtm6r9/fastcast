use crate::stream::FALLBACK_STREAM_FPS;
use std::time::Duration;

pub fn frame_interval_of(fps: u32) -> Duration {
    let rate = if fps == 0 { FALLBACK_STREAM_FPS } else { fps };

    Duration::from_secs(1) / rate
}

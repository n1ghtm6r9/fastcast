use crate::capture::FALLBACK_CAPTURE_FPS;
use crate::settings::FrameRate;

pub fn monitor_fps_for(frame_rate: FrameRate, refresh_hz: Option<u32>) -> u32 {
    match frame_rate.limit() {
        Some(limit) => refresh_hz.map_or(limit, |refresh_hz| limit.min(refresh_hz)),
        None => refresh_hz.unwrap_or(FALLBACK_CAPTURE_FPS),
    }
}

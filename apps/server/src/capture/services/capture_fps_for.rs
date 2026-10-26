use crate::display::DisplayMode;
use crate::settings::FrameRate;

pub fn capture_fps_for(requested: DisplayMode, shown: DisplayMode, frame_rate: FrameRate) -> u32 {
    let refresh_hz = requested.refresh_hz.min(shown.refresh_hz);
    frame_rate
        .limit()
        .map_or(refresh_hz, |limit| limit.min(refresh_hz))
}

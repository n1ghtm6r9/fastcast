use crate::capture::BITRATE_REFERENCE_FPS;

pub fn stream_bitrate_kbps(base_kbps: u32, fps: u32) -> u32 {
    if fps <= BITRATE_REFERENCE_FPS {
        return base_kbps;
    }

    let scale = (f64::from(fps) / f64::from(BITRATE_REFERENCE_FPS)).sqrt();

    (f64::from(base_kbps) * scale).round() as u32
}

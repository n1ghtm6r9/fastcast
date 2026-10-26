use crate::capture::{FAST_ENCODER_PRESET, MAX_QUALITY_PRESET_PIXEL_RATE, QUALITY_ENCODER_PRESET};

pub fn encoder_preset_for(width: u32, height: u32, fps: u32) -> u8 {
    let pixel_rate = u64::from(width) * u64::from(height) * u64::from(fps);

    if pixel_rate > MAX_QUALITY_PRESET_PIXEL_RATE {
        FAST_ENCODER_PRESET
    } else {
        QUALITY_ENCODER_PRESET
    }
}

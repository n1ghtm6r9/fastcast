use crate::capture::{FAST_ENCODER_PRESET, NativePipeline, encoder_codec_of};
use nvenc::EncoderSettings;

pub fn encoder_settings_for(
    pipeline: NativePipeline,
    size: (u32, u32),
    fps: u32,
    bitrate_kbps: u32,
) -> EncoderSettings {
    EncoderSettings {
        codec: encoder_codec_of(pipeline.codec),
        width: size.0,
        height: size.1,
        fps,
        bitrate_kbps,
        bit_depth: pipeline.bit_depth,
        preset: FAST_ENCODER_PRESET,
    }
}

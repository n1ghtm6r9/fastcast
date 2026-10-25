use crate::encoder::{EncoderCodec, EncoderSettings};

pub fn settings(codec: EncoderCodec, width: u32, height: u32, bit_depth: u8) -> EncoderSettings {
    EncoderSettings {
        codec,
        width,
        height,
        fps: 60,
        bitrate_kbps: 50_000,
        bit_depth,
        preset: 4,
    }
}

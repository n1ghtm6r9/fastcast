use crate::encoder::EncoderCodec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncoderSettings {
    pub codec: EncoderCodec,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub bit_depth: u8,
    pub preset: u8,
}

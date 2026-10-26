use crate::av1::Av1ColorConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Av1SequenceHeader {
    pub profile: u8,
    pub level: u8,
    pub tier: bool,
    pub max_frame_width: u32,
    pub max_frame_height: u32,
    pub color: Av1ColorConfig,
}

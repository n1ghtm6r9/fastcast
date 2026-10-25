#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FcSettings {
    pub codec: u32,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
    pub bit_depth: u32,
    pub preset: u32,
}

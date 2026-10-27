use crate::settings::{CodecChoice, FrameRate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub server_name: String,
    pub virtual_display: bool,
    pub capture_output: u32,
    pub frame_rate: FrameRate,
    pub video_codec: CodecChoice,
    pub video_bitrate_kbps: u32,
    pub debug: bool,
}

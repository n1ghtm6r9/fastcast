use crate::settings::{BackendChoice, EncoderChoice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub server_name: String,
    pub ffmpeg_path: String,
    pub video_encoder: EncoderChoice,
    pub capture_backend: BackendChoice,
    pub virtual_display: bool,
    pub capture_output: u32,
    pub capture_fps: u32,
    pub video_bitrate_kbps: u32,
    pub keyframe_interval_seconds: u32,
}

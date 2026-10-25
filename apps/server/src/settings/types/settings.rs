use crate::settings::EncoderChoice;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub web_host: String,
    pub web_port: u16,
    pub server_name: String,
    pub ffmpeg_path: String,
    pub video_encoder: EncoderChoice,
    pub virtual_display: bool,
    pub capture_output: u32,
    pub capture_fps: u32,
    pub video_bitrate_kbps: u32,
    pub keyframe_interval_seconds: u32,
}

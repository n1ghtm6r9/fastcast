use crate::settings::constants::{
    CAPTURE_FPS_SETTING, CAPTURE_OUTPUT_SETTING, KEYFRAME_INTERVAL_SECONDS_SETTING,
    VIDEO_BITRATE_KBPS_SETTING,
};
use crate::settings::{BackendChoice, EncoderChoice, LogLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SavedSettings {
    pub server_name: String,
    pub capture_backend: BackendChoice,
    pub video_encoder: EncoderChoice,
    pub virtual_display: bool,
    pub capture_output: u32,
    pub capture_fps: u32,
    pub video_bitrate_kbps: u32,
    pub ten_bit_video: bool,
    pub keyframe_interval_seconds: u32,
    pub ffmpeg_path: String,
    pub log_level: LogLevel,
}

impl Default for SavedSettings {
    fn default() -> Self {
        Self {
            server_name: String::new(),
            capture_backend: BackendChoice::Auto,
            video_encoder: EncoderChoice::Auto,
            virtual_display: true,
            capture_output: CAPTURE_OUTPUT_SETTING.default,
            capture_fps: CAPTURE_FPS_SETTING.default,
            video_bitrate_kbps: VIDEO_BITRATE_KBPS_SETTING.default,
            ten_bit_video: true,
            keyframe_interval_seconds: KEYFRAME_INTERVAL_SECONDS_SETTING.default,
            ffmpeg_path: String::new(),
            log_level: LogLevel::default(),
        }
    }
}

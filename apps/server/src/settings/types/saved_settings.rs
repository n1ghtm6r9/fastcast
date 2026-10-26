use crate::settings::constants::{CAPTURE_OUTPUT_SETTING, VIDEO_BITRATE_MBPS_SETTING};
use crate::settings::{CodecChoice, FrameRate, LogLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SavedSettings {
    pub server_name: String,
    pub virtual_display: bool,
    pub capture_output: u32,
    pub frame_rate: FrameRate,
    pub video_codec: CodecChoice,
    pub video_bitrate_mbps: u32,
    pub log_level: LogLevel,
}

impl Default for SavedSettings {
    fn default() -> Self {
        Self {
            server_name: String::new(),
            virtual_display: true,
            capture_output: CAPTURE_OUTPUT_SETTING.default,
            frame_rate: FrameRate::default(),
            video_codec: CodecChoice::default(),
            video_bitrate_mbps: VIDEO_BITRATE_MBPS_SETTING.default,
            log_level: LogLevel::default(),
        }
    }
}

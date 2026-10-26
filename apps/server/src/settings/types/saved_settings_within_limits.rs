use crate::settings::SavedSettings;
use crate::settings::constants::{
    CAPTURE_FPS_SETTING, CAPTURE_OUTPUT_SETTING, KEYFRAME_INTERVAL_SECONDS_SETTING,
    VIDEO_BITRATE_KBPS_SETTING,
};

impl SavedSettings {
    pub fn within_limits(self) -> Self {
        Self {
            capture_output: CAPTURE_OUTPUT_SETTING.clamped(self.capture_output),
            capture_fps: CAPTURE_FPS_SETTING.clamped(self.capture_fps),
            video_bitrate_kbps: VIDEO_BITRATE_KBPS_SETTING.clamped(self.video_bitrate_kbps),
            keyframe_interval_seconds: KEYFRAME_INTERVAL_SECONDS_SETTING
                .clamped(self.keyframe_interval_seconds),
            ..self
        }
    }
}

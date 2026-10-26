use crate::settings::SavedSettings;
use crate::settings::constants::{CAPTURE_OUTPUT_SETTING, VIDEO_BITRATE_MBPS_SETTING};

impl SavedSettings {
    pub fn within_limits(self) -> Self {
        Self {
            capture_output: CAPTURE_OUTPUT_SETTING.clamped(self.capture_output),
            video_bitrate_mbps: VIDEO_BITRATE_MBPS_SETTING.clamped(self.video_bitrate_mbps),
            ..self
        }
    }
}

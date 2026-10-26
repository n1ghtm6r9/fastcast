use crate::settings::constants::{DEFAULT_SERVER_NAME, KILOBITS_PER_MEGABIT};
use crate::settings::{SavedSettings, Settings, trimmed_or};

pub fn settings_of(saved: &SavedSettings) -> Settings {
    Settings {
        server_name: trimmed_or(&saved.server_name, DEFAULT_SERVER_NAME),
        virtual_display: saved.virtual_display,
        capture_output: saved.capture_output,
        frame_rate: saved.frame_rate,
        video_codec: saved.video_codec,
        video_bitrate_kbps: saved
            .video_bitrate_mbps
            .saturating_mul(KILOBITS_PER_MEGABIT),
    }
}

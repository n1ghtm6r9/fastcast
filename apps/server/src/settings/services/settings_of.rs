use crate::settings::constants::FFMPEG_SEARCH_PATH;
use crate::settings::{SavedSettings, Settings, trimmed_or};

pub fn settings_of(saved: &SavedSettings, computer_name: &str) -> Settings {
    Settings {
        server_name: trimmed_or(&saved.server_name, computer_name),
        ffmpeg_path: trimmed_or(&saved.ffmpeg_path, FFMPEG_SEARCH_PATH),
        video_encoder: saved.video_encoder,
        capture_backend: saved.capture_backend,
        virtual_display: saved.virtual_display,
        capture_output: saved.capture_output,
        capture_fps: saved.capture_fps,
        video_bitrate_kbps: saved.video_bitrate_kbps,
        ten_bit_video: saved.ten_bit_video,
        keyframe_interval_seconds: saved.keyframe_interval_seconds,
    }
}

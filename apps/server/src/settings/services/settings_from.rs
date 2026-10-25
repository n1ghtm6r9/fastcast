use crate::settings::constants::{
    CAPTURE_FPS_SETTING, CAPTURE_OUTPUT_SETTING, FFMPEG_PATH_SETTING,
    KEYFRAME_INTERVAL_SECONDS_SETTING, VIDEO_BITRATE_KBPS_SETTING, VIDEO_ENCODER_SETTING,
    VIRTUAL_DISPLAY_SETTING, WEB_HOST_SETTING, WEB_PORT_SETTING,
};
use crate::settings::{
    Settings, SettingsError, read_encoder_choice, read_number_setting, read_server_name,
    read_switch_setting, read_text_setting,
};

pub fn settings_from(lookup: impl Fn(&str) -> Option<String>) -> Result<Settings, SettingsError> {
    Ok(Settings {
        web_host: read_text_setting(&WEB_HOST_SETTING, &lookup),
        web_port: read_number_setting(&WEB_PORT_SETTING, &lookup)? as u16,
        server_name: read_server_name(&lookup),
        ffmpeg_path: read_text_setting(&FFMPEG_PATH_SETTING, &lookup),
        video_encoder: read_encoder_choice(&VIDEO_ENCODER_SETTING, &lookup)?,
        virtual_display: read_switch_setting(&VIRTUAL_DISPLAY_SETTING, &lookup)?,
        capture_output: read_number_setting(&CAPTURE_OUTPUT_SETTING, &lookup)?,
        capture_fps: read_number_setting(&CAPTURE_FPS_SETTING, &lookup)?,
        video_bitrate_kbps: read_number_setting(&VIDEO_BITRATE_KBPS_SETTING, &lookup)?,
        keyframe_interval_seconds: read_number_setting(
            &KEYFRAME_INTERVAL_SECONDS_SETTING,
            &lookup,
        )?,
    })
}

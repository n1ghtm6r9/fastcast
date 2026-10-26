use crate::settings::SavedSettings;
use crate::settings::constants::{
    CAPTURE_FPS_SETTING, KEYFRAME_INTERVAL_SECONDS_SETTING, VIDEO_BITRATE_KBPS_SETTING,
};
use crate::window::test::{
    applied_settings, press_button, running_status, saving_works, set_number, test_window,
    window_harness,
};

#[test]
fn numbers_below_their_limits_are_applied_as_the_limits() {
    let saved = SavedSettings {
        keyframe_interval_seconds: 5,
        ..SavedSettings::default()
    };
    let (window, probe) = test_window(saved, running_status(), saving_works);
    let mut harness = window_harness(window);

    set_number(&mut harness, "Кадров в секунду", 0.0);
    set_number(&mut harness, "Битрейт, кбит/с", 0.0);
    set_number(&mut harness, "Ключевой кадр, с", 0.0);
    press_button(&mut harness, "Применить");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            capture_fps: CAPTURE_FPS_SETTING.min,
            video_bitrate_kbps: VIDEO_BITRATE_KBPS_SETTING.min,
            keyframe_interval_seconds: KEYFRAME_INTERVAL_SECONDS_SETTING.min,
            ..SavedSettings::default()
        }]
    );
}

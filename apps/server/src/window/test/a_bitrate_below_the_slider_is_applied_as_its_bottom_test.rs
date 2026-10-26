use crate::settings::SavedSettings;
use crate::settings::constants::VIDEO_BITRATE_MBPS_SETTING;
use crate::window::test::{
    applied_settings, running_status, saving_works, slide_to, test_window, window_harness,
};

#[test]
fn a_bitrate_below_the_slider_is_applied_as_its_bottom() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    slide_to(&mut harness, "Битрейт", 0.0);

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            video_bitrate_mbps: VIDEO_BITRATE_MBPS_SETTING.min,
            ..SavedSettings::default()
        }]
    );
}

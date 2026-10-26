use crate::settings::SavedSettings;
use crate::window::test::{
    SECOND_MONITOR_PLUGGED, applied_settings, choose_option, monitors_as_plugged, running_status,
    saving_works, test_window_listing, window_harness,
};

#[test]
fn a_monitor_plugged_in_while_the_window_is_open_can_be_chosen() {
    let (window, probe) = test_window_listing(
        SavedSettings::default(),
        running_status(),
        saving_works,
        monitors_as_plugged,
    );
    let mut harness = window_harness(window);

    SECOND_MONITOR_PLUGGED.set(true);
    choose_option(&mut harness, "Экран", "1: DISPLAY2, 1920×1080");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            virtual_display: false,
            capture_output: 1,
            ..SavedSettings::default()
        }]
    );
}

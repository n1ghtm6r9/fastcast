use crate::settings::{FrameRate, SavedSettings};
use crate::window::test::{
    applied_settings, choose_option, running_status, saving_works, test_window, window_harness,
};

#[test]
fn applied_settings_are_not_applied_twice() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    choose_option(&mut harness, "Кадров/с", "30");
    harness.run();

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            frame_rate: FrameRate::Fps30,
            ..SavedSettings::default()
        }]
    );
}

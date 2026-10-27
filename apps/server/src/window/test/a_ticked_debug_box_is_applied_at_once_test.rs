use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, running_status, saving_works, test_window, tick, window_harness,
};

#[test]
fn a_ticked_debug_box_is_applied_at_once() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    tick(&mut harness, "Отладка");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            debug: true,
            ..SavedSettings::default()
        }]
    );
}

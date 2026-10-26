use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, choose_option, running_status, saving_works, test_window, window_harness,
};

#[test]
fn the_virtual_screen_can_be_chosen_again() {
    let saved = SavedSettings {
        virtual_display: false,
        capture_output: 1,
        ..SavedSettings::default()
    };
    let (window, probe) = test_window(saved.clone(), running_status(), saving_works);
    let mut harness = window_harness(window);

    choose_option(&mut harness, "Экран", "Виртуальный под ТВ");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            virtual_display: true,
            ..saved
        }]
    );
}

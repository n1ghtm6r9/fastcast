use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, press_button, running_status, saving_works, test_window, window_harness,
};

#[test]
fn defaults_fill_the_form_without_applying_it() {
    let saved = SavedSettings {
        server_name: "Office PC".to_owned(),
        capture_fps: 120,
        virtual_display: false,
        ..SavedSettings::default()
    };
    let (window, probe) = test_window(saved, running_status(), saving_works);
    let mut harness = window_harness(window);

    press_button(&mut harness, "По умолчанию");
    let applied_by_defaults = applied_settings(&probe);
    press_button(&mut harness, "Применить");

    assert!(applied_by_defaults.is_empty());
    assert_eq!(applied_settings(&probe), vec![SavedSettings::default()]);
}

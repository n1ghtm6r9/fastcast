use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, named_settings, press_button, running_status, saving_works, test_window,
    type_into, window_harness,
};

#[test]
fn applying_hands_the_edited_settings_to_the_server() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя сервера", "Gaming PC");
    press_button(&mut harness, "Применить");

    assert_eq!(applied_settings(&probe), vec![named_settings("Gaming PC")]);
}

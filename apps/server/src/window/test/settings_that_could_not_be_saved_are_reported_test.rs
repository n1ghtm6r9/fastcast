use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, named_settings, press_button, running_status, saving_fails, test_window,
    type_into, window_harness,
};
use egui_kittest::kittest::Queryable;

#[test]
fn settings_that_could_not_be_saved_are_reported() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_fails);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя сервера", "Gaming PC");
    press_button(&mut harness, "Применить");

    assert_eq!(applied_settings(&probe), vec![named_settings("Gaming PC")]);
    harness.get_by_label(&format!(
        "Настройки применены, но не сохранены: {}",
        saving_fails().unwrap_err()
    ));
}

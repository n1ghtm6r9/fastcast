use crate::settings::SavedSettings;
use crate::window::test::{
    choose_option, running_status, saving_fails, test_window, window_harness,
};
use egui_kittest::kittest::Queryable;

#[test]
fn settings_that_could_not_be_saved_are_reported() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_fails);
    let mut harness = window_harness(window);

    choose_option(&mut harness, "Кадров/с", "30");

    harness.get_by_label(&format!(
        "Настройки применены, но не сохранены: {}",
        saving_fails().unwrap_err()
    ));
}

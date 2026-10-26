use crate::window::test::{
    applied_settings, named_settings, press_button, running_status, saving_works, test_window,
    type_into, window_harness,
};
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

#[test]
fn cancelling_returns_the_form_to_the_applied_settings() {
    let (window, probe) = test_window(named_settings("Office PC"), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя сервера", " 2");
    press_button(&mut harness, "Отменить");
    press_button(&mut harness, "Применить");

    assert_eq!(
        harness
            .get_by_role_and_label(Role::TextInput, "Имя сервера")
            .value(),
        Some("Office PC".to_owned())
    );
    assert!(applied_settings(&probe).is_empty());
}

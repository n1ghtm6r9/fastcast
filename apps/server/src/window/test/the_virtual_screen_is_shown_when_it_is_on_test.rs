use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

#[test]
fn the_virtual_screen_is_shown_when_it_is_on() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);

    let harness = window_harness(window);

    assert_eq!(
        harness
            .get_by_role_and_label(Role::ComboBox, "Экран")
            .value(),
        Some("Виртуальный под ТВ".to_owned())
    );
}

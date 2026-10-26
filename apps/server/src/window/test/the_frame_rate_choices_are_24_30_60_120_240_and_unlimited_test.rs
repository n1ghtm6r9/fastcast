use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

#[test]
fn the_frame_rate_choices_are_24_30_60_120_240_and_unlimited() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    harness
        .get_by_role_and_label(Role::ComboBox, "Кадров/с")
        .click();
    harness.run();

    for option in ["24", "30", "60", "120", "240", "Без ограничения"] {
        harness.get_by_role_and_label(Role::Button, option);
    }
}

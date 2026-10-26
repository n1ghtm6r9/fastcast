use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn choose_option(harness: &mut WindowTestHarness, choice: &str, option: &str) {
    harness
        .get_by_role_and_label(Role::ComboBox, choice)
        .click();
    harness.run();
    harness.get_by_role_and_label(Role::Button, option).click();
    harness.run();
}

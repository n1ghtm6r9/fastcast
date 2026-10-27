use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn tick(harness: &mut WindowTestHarness, label: &str) {
    harness.get_by_role_and_label(Role::CheckBox, label).click();
    harness.run();
}

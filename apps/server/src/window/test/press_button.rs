use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn press_button(harness: &mut WindowTestHarness, label: &str) {
    harness.get_by_role_and_label(Role::Button, label).click();
    harness.run();
}

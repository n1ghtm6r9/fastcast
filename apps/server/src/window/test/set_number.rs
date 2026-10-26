use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn set_number(harness: &mut WindowTestHarness, label: &str, number: f64) {
    harness
        .get_by_role_and_label(Role::SpinButton, label)
        .set_value_accesskit(number);
    harness.run();
}

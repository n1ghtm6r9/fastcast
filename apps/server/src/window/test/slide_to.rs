use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn slide_to(harness: &mut WindowTestHarness, label: &str, value: f64) {
    harness
        .get_by_role_and_label(Role::Slider, label)
        .set_value_accesskit(value);
    harness.run();
}

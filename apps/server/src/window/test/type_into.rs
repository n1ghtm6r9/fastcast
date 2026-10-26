use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn type_into(harness: &mut WindowTestHarness, field: &str, text: &str) {
    harness
        .get_by_role_and_label(Role::TextInput, field)
        .focus();
    harness.run();
    harness
        .get_by_role_and_label(Role::TextInput, field)
        .type_text(text);
    harness.run();
}

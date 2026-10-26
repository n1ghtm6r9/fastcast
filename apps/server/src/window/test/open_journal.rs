use crate::window::test::WindowTestHarness;
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

pub fn open_journal(harness: &mut WindowTestHarness) {
    harness
        .get_by_role_and_label(Role::DisclosureTriangle, "Журнал")
        .click();
    harness.run();
}

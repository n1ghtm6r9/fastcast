use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use eframe::egui::Role;
use egui_kittest::kittest::{NodeT, Queryable};

#[test]
fn a_blank_name_shows_pc_in_its_place() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);

    let harness = window_harness(window);

    assert_eq!(
        harness
            .get_by_role_and_label(Role::TextInput, "Имя на ТВ")
            .accesskit_node()
            .placeholder(),
        Some("PC")
    );
}

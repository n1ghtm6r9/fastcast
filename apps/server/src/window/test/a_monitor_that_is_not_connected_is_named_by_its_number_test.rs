use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

#[test]
fn a_monitor_that_is_not_connected_is_named_by_its_number() {
    let saved = SavedSettings {
        capture_output: 3,
        ..SavedSettings::default()
    };
    let (window, _probe) = test_window(saved, running_status(), saving_works);

    let harness = window_harness(window);

    assert_eq!(
        harness
            .get_by_role_and_label(Role::ComboBox, "Монитор")
            .value(),
        Some("3: не подключён".to_owned())
    );
}

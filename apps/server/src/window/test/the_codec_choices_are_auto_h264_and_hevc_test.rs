use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use eframe::egui::Role;
use egui_kittest::kittest::Queryable;

#[test]
fn the_codec_choices_are_auto_h264_and_hevc() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    harness
        .get_by_role_and_label(Role::ComboBox, "Кодек")
        .click();
    harness.run();

    for option in ["Авто", "H.264", "HEVC"] {
        harness.get_by_role_and_label(Role::Button, option);
    }
}

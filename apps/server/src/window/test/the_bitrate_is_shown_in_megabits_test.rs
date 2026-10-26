use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_bitrate_is_shown_in_megabits() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);

    let harness = window_harness(window);

    harness.get_by_label("60 Мбит/с");
}

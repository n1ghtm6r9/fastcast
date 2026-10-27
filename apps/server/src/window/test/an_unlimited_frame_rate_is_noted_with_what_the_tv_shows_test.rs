use crate::settings::SavedSettings;
use crate::window::test::{saving_works, test_window, watching_at, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn an_unlimited_frame_rate_is_noted_with_what_the_tv_shows() {
    let (window, _probe) = test_window(SavedSettings::default(), watching_at(100), saving_works);

    let harness = window_harness(window);

    harness.get_by_label("ТВ показывает не больше 100 кадров/с");
}

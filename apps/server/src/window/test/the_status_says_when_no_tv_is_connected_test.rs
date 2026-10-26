use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_says_when_no_tv_is_connected() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);

    let harness = window_harness(window);

    harness.get_by_label("ТВ не подключён");
}

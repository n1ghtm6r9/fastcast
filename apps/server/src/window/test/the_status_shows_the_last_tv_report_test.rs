use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_shows_the_last_tv_report() {
    let watched = ServerStatus {
        report: Some("decoded 119 fps".to_owned()),
        ..running_status()
    };
    let (window, _probe) = test_window(SavedSettings::default(), watched, saving_works);

    let harness = window_harness(window);

    harness.get_by_label("Отчёт ТВ: decoded 119 fps");
}

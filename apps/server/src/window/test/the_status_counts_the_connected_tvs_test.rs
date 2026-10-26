use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_counts_the_connected_tvs() {
    let watched = ServerStatus {
        viewers: 2,
        ..running_status()
    };
    let (window, _probe) = test_window(SavedSettings::default(), watched, saving_works);

    let harness = window_harness(window);

    harness.get_by_label("Транслирует на 2 ТВ");
}

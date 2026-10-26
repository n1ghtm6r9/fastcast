use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_shows_that_the_server_is_starting() {
    let (window, _probe) = test_window(
        SavedSettings::default(),
        ServerStatus::default(),
        saving_works,
    );

    let harness = window_harness(window);

    harness.get_by_label("Запускается…");
}

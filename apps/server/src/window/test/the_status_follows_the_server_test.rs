use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_follows_the_server() {
    let (window, probe) = test_window(
        SavedSettings::default(),
        ServerStatus::default(),
        saving_works,
    );
    let mut harness = window_harness(window);

    probe.status.send_replace(running_status());
    harness.run();

    harness.get_by_label("Ждёт ТВ");
}

use crate::server::{ServerState, ServerStatus};
use crate::settings::SavedSettings;
use crate::window::test::{saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_status_shows_why_the_server_failed() {
    let failed = ServerStatus {
        state: ServerState::Failed {
            reason: "udp 0.0.0.0:3278 is busy".to_owned(),
        },
        ..ServerStatus::default()
    };
    let (window, _probe) = test_window(SavedSettings::default(), failed, saving_works);

    let harness = window_harness(window);

    harness.get_by_label("Не запустился");
    harness.get_by_label("udp 0.0.0.0:3278 is busy");
}

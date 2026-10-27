use crate::server::ServerStatus;
use crate::settings::SavedSettings;
use crate::window::test::{saving_works, test_window, watching_at, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn no_frame_rate_note_is_shown_while_no_tv_is_connected() {
    let left = ServerStatus {
        viewers: 0,
        ..watching_at(60)
    };
    let (window, _probe) = test_window(SavedSettings::default(), left, saving_works);

    let harness = window_harness(window);

    assert!(
        harness
            .query_by_label("ТВ показывает не больше 60 кадров/с")
            .is_none()
    );
}

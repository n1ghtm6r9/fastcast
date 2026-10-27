use crate::settings::{FrameRate, SavedSettings};
use crate::window::test::{saving_works, test_window, watching_at, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn a_frame_rate_the_tv_shows_is_not_noted() {
    let saved = SavedSettings {
        frame_rate: FrameRate::Fps60,
        ..SavedSettings::default()
    };
    let (window, _probe) = test_window(saved, watching_at(60), saving_works);

    let harness = window_harness(window);

    assert!(
        harness
            .query_by_label("ТВ показывает не больше 60 кадров/с")
            .is_none()
    );
}

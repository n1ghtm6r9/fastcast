use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn no_request_is_shown_while_no_tv_is_connected() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);

    let harness = window_harness(window);

    assert!(
        harness
            .query_by_label("ТВ просит картинку по настройкам")
            .is_none()
    );
}

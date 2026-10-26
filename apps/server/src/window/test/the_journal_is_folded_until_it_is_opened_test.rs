use crate::settings::SavedSettings;
use crate::window::test::{running_status, saving_works, test_window, window_harness};
use egui_kittest::kittest::Queryable;

#[test]
fn the_journal_is_folded_until_it_is_opened() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    probe.lines.push("first line".to_owned());

    let harness = window_harness(window);

    assert!(harness.query_by_label("first line").is_none());
}

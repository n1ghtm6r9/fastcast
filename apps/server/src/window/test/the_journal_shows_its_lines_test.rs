use crate::settings::SavedSettings;
use crate::window::test::{
    open_journal, running_status, saving_works, test_window, window_harness,
};
use egui_kittest::kittest::Queryable;

#[test]
fn the_journal_shows_its_lines() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    probe
        .lines
        .push("12:00:00.000  INFO fastcast is listening on udp 192.168.0.10:3278".to_owned());
    let mut harness = window_harness(window);

    open_journal(&mut harness);

    harness.get_by_label("12:00:00.000  INFO fastcast is listening on udp 192.168.0.10:3278");
}

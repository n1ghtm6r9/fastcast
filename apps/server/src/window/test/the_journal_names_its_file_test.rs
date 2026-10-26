use crate::settings::SavedSettings;
use crate::window::test::{
    TEST_JOURNAL_FILE, press_button, running_status, saving_works, test_window, window_harness,
};
use egui_kittest::kittest::Queryable;

#[test]
fn the_journal_names_its_file() {
    let (window, _probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    press_button(&mut harness, "Журнал");

    harness.get_by_label(&format!("Файл журнала: {TEST_JOURNAL_FILE}"));
}

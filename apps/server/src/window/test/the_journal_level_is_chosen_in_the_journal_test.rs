use crate::settings::{LogLevel, SavedSettings};
use crate::window::test::{
    applied_settings, choose_option, open_journal, running_status, saving_works, test_window,
    window_harness,
};

#[test]
fn the_journal_level_is_chosen_in_the_journal() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    open_journal(&mut harness);
    choose_option(&mut harness, "Подробность", "Подробная");

    assert_eq!(
        applied_settings(&probe),
        vec![SavedSettings {
            log_level: LogLevel::Debug,
            ..SavedSettings::default()
        }]
    );
}

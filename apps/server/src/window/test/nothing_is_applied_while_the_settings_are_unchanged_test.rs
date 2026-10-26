use crate::window::test::{
    applied_settings, named_settings, running_status, saving_works, test_window, window_harness,
};

#[test]
fn nothing_is_applied_while_the_settings_are_unchanged() {
    let (window, probe) = test_window(named_settings("Office PC"), running_status(), saving_works);
    let mut harness = window_harness(window);

    harness.run();

    assert!(applied_settings(&probe).is_empty());
}

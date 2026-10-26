use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, running_status, saving_works, test_window, type_into, window_harness,
};

#[test]
fn a_name_is_not_applied_while_it_is_typed() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя на ТВ", "Gaming PC");

    assert!(applied_settings(&probe).is_empty());
}

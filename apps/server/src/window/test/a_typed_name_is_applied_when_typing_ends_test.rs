use crate::settings::SavedSettings;
use crate::window::test::{
    applied_settings, finish_typing, named_settings, running_status, saving_works, test_window,
    type_into, window_harness,
};

#[test]
fn a_typed_name_is_applied_when_typing_ends() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    let mut harness = window_harness(window);

    type_into(&mut harness, "Имя на ТВ", "Gaming PC");
    finish_typing(&mut harness);

    assert_eq!(applied_settings(&probe), vec![named_settings("Gaming PC")]);
}

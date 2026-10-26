use crate::settings::SavedSettings;
use crate::window::test::{
    press_button, running_status, saving_works, test_window, window_harness,
};
use eframe::egui::{OutputCommand, Role};
use egui_kittest::kittest::Queryable;

#[test]
fn copying_the_journal_puts_its_lines_on_the_clipboard() {
    let (window, probe) = test_window(SavedSettings::default(), running_status(), saving_works);
    probe.lines.push("first line".to_owned());
    probe.lines.push("second line".to_owned());
    let mut harness = window_harness(window);

    press_button(&mut harness, "Журнал");
    harness
        .get_by_role_and_label(Role::Button, "Копировать")
        .click();
    harness.step();

    assert!(
        harness
            .output()
            .platform_output
            .commands
            .contains(&OutputCommand::CopyText(
                "first line\nsecond line".to_owned()
            ))
    );
}

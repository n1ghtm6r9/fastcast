use crate::window::test::WindowTestHarness;
use eframe::egui::Key;

pub fn finish_typing(harness: &mut WindowTestHarness) {
    harness.key_press(Key::Enter);
    harness.run();
}

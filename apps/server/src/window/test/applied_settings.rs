use crate::settings::SavedSettings;
use crate::window::test::WindowProbe;

pub fn applied_settings(probe: &WindowProbe) -> Vec<SavedSettings> {
    probe.applied.borrow().clone()
}

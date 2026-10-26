use crate::settings::SavedSettings;
use crate::window::Monitor;

pub struct SettingsForm {
    pub(crate) applied: SavedSettings,
    pub(crate) edited: SavedSettings,
    pub(crate) editing: bool,
    pub(crate) monitors: Vec<Monitor>,
    pub(crate) list_monitors: fn() -> Vec<Monitor>,
    pub(crate) problem: Option<String>,
}

impl SettingsForm {
    pub fn new(saved: SavedSettings, list_monitors: fn() -> Vec<Monitor>) -> Self {
        Self {
            edited: saved.clone(),
            applied: saved,
            editing: false,
            monitors: list_monitors(),
            list_monitors,
            problem: None,
        }
    }
}

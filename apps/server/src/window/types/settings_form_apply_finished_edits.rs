use crate::window::{ServerLink, SettingsForm};

impl SettingsForm {
    pub fn apply_finished_edits(&mut self, link: &mut ServerLink<'_>) {
        if !self.editing && self.edited != self.applied {
            self.apply(link);
        }
    }
}

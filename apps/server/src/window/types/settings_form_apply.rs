use crate::window::{ServerLink, SettingsForm};

impl SettingsForm {
    pub fn apply(&mut self, link: &mut ServerLink<'_>) {
        self.problem = link
            .apply(&self.edited)
            .err()
            .map(|problem| format!("Настройки применены, но не сохранены: {problem}"));
        self.applied = self.edited.clone();
    }
}

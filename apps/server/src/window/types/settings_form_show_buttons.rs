use crate::settings::SavedSettings;
use crate::window::{ServerLink, SettingsForm};
use eframe::egui::{Button, Ui};

impl SettingsForm {
    pub fn show_buttons(&mut self, ui: &mut Ui, link: &mut ServerLink<'_>, failed: bool) {
        if let Some(problem) = &self.problem {
            ui.colored_label(ui.visuals().error_fg_color, problem);
        }

        let changed = self.edited != self.applied;
        ui.horizontal(|ui| {
            if ui
                .add_enabled(changed || failed, Button::new("Применить"))
                .clicked()
            {
                self.problem = link
                    .apply(&self.edited)
                    .err()
                    .map(|problem| format!("Настройки применены, но не сохранены: {problem}"));
                self.applied = self.edited.clone();
            }
            if ui.add_enabled(changed, Button::new("Отменить")).clicked() {
                self.edited = self.applied.clone();
            }
            if ui.button("По умолчанию").clicked() {
                self.edited = SavedSettings::default();
            }
        });
    }
}

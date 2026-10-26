use crate::window::{LEVEL_CHOICES, SettingsForm, choice_combo};
use eframe::egui::Ui;

impl SettingsForm {
    pub fn show_level(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let label = ui.label("Подробность");
            choice_combo(ui, "log_level", &mut self.edited.log_level, &LEVEL_CHOICES)
                .labelled_by(label.id);
        });
    }
}

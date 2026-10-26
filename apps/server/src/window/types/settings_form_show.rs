use crate::window::SettingsForm;
use eframe::egui::{Grid, ScrollArea, Ui};

impl SettingsForm {
    pub fn show(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Grid::new("settings")
                    .num_columns(2)
                    .spacing([24.0, 10.0])
                    .show(ui, |ui| self.show_rows(ui));
            });
    }
}

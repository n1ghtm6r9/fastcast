use crate::server::ServerState;
use crate::window::{ServerWindow, WindowTab, show_status};
use eframe::egui::{CentralPanel, Panel, Ui};

impl ServerWindow<'_> {
    pub fn show(&mut self, ui: &mut Ui) {
        let status = self.link.status();

        Panel::top("status").show(ui, |ui| show_status(ui, &status));
        Panel::top("tabs").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, WindowTab::Settings, "Настройки");
                ui.selectable_value(&mut self.tab, WindowTab::Journal, "Журнал");
            });
        });

        match self.tab {
            WindowTab::Settings => {
                let failed = matches!(status.state, ServerState::Failed { .. });

                Panel::bottom("buttons")
                    .show(ui, |ui| self.form.show_buttons(ui, &mut self.link, failed));
                CentralPanel::default().show(ui, |ui| self.form.show(ui));
            }
            WindowTab::Journal => {
                CentralPanel::default().show(ui, |ui| self.journal.show(ui));
            }
        }
    }
}

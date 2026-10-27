use crate::window::{ServerWindow, show_status, tv_refresh_hz};
use eframe::egui::{CentralPanel, CollapsingHeader, Frame, Margin, Panel, Ui};

impl ServerWindow<'_> {
    pub fn show(&mut self, ui: &mut Ui) {
        let status = self.link.status();
        let header = Frame::side_top_panel(ui.style()).inner_margin(Margin::symmetric(16, 12));
        let body = Frame::central_panel(ui.style()).inner_margin(Margin::same(16));

        Panel::top("status").frame(header).show(ui, |ui| {
            if show_status(ui, &status) {
                self.form.apply(&mut self.link);
            }
        });
        CentralPanel::default().frame(body).show(ui, |ui| {
            self.form.show(ui, tv_refresh_hz(&status));
            ui.separator();
            CollapsingHeader::new("Журнал").show(ui, |ui| {
                self.form.show_level(ui);
                self.journal.show(ui);
            });
        });

        self.form.apply_finished_edits(&mut self.link);
    }
}

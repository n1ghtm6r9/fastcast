use crate::window::JournalView;
use eframe::egui::{Label, RichText, ScrollArea, TextStyle, Ui};

impl JournalView {
    pub fn show(&self, ui: &mut Ui) {
        let lines = self.lines.lines();

        ui.horizontal(|ui| {
            ui.label(format!("Файл журнала: {}", self.file.display()));
            if ui.button("Копировать").clicked() {
                ui.ctx().copy_text(lines.join("\n"));
            }
        });
        ui.separator();

        let row_height = ui.text_style_height(&TextStyle::Monospace);
        ScrollArea::both()
            .auto_shrink([false, false])
            .stick_to_bottom(true)
            .show_rows(ui, row_height, lines.len(), |ui, rows| {
                for line in &lines[rows] {
                    ui.add(Label::new(RichText::new(line).monospace()).extend());
                }
            });
    }
}

use eframe::egui::{Response, RichText, Ui};

pub fn setting_row(ui: &mut Ui, name: &str, hint: &str, widget: impl FnOnce(&mut Ui) -> Response) {
    let label = ui.label(name);

    ui.vertical(|ui| {
        widget(ui).labelled_by(label.id);
        ui.label(RichText::new(hint).weak());
    });
    ui.end_row();
}

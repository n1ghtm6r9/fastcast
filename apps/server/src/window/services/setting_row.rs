use eframe::egui::{Response, Ui};

pub fn setting_row(ui: &mut Ui, name: &str, widget: impl FnOnce(&mut Ui) -> Response) -> Response {
    let label = ui.label(name);
    let response = widget(ui).labelled_by(label.id);

    ui.end_row();
    response
}

use crate::window::{Monitor, monitor_text};
use eframe::egui::{ComboBox, Response, Ui};

pub fn monitor_combo(
    ui: &mut Ui,
    output: &mut u32,
    monitors: &mut Vec<Monitor>,
    list_monitors: fn() -> Vec<Monitor>,
) -> Response {
    let response = ComboBox::from_id_salt("capture_output")
        .selected_text(monitor_text(*output, monitors))
        .show_ui(ui, |ui| {
            for monitor in monitors.iter() {
                ui.selectable_value(output, monitor.index, monitor_text(monitor.index, monitors));
            }
        })
        .response;

    if response.clicked() {
        *monitors = list_monitors();
    }

    response
}

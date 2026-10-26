use crate::settings::SavedSettings;
use crate::window::{FIELD_WIDTH, Monitor, VIRTUAL_SCREEN_TEXT, monitor_text};
use eframe::egui::{ComboBox, Response, Ui};

pub fn screen_combo(
    ui: &mut Ui,
    settings: &mut SavedSettings,
    monitors: &mut Vec<Monitor>,
    list_monitors: fn() -> Vec<Monitor>,
) -> Response {
    let shown = if settings.virtual_display {
        VIRTUAL_SCREEN_TEXT.to_owned()
    } else {
        monitor_text(settings.capture_output, monitors)
    };
    let response = ComboBox::from_id_salt("screen")
        .width(FIELD_WIDTH)
        .selected_text(shown)
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(settings.virtual_display, VIRTUAL_SCREEN_TEXT)
                .clicked()
            {
                settings.virtual_display = true;
            }
            for monitor in monitors.iter() {
                let chosen = !settings.virtual_display && settings.capture_output == monitor.index;
                if ui
                    .selectable_label(chosen, monitor_text(monitor.index, monitors))
                    .clicked()
                {
                    settings.virtual_display = false;
                    settings.capture_output = monitor.index;
                }
            }
        })
        .response;

    if response.clicked() {
        *monitors = list_monitors();
    }

    response
}

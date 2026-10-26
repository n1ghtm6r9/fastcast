use crate::server::{ServerState, ServerStatus};
use crate::window::{request_text, state_text, viewers_text};
use eframe::egui::{RichText, Ui};

pub fn show_status(ui: &mut Ui, status: &ServerStatus) {
    let state = RichText::new(state_text(&status.state)).strong();

    match status.state {
        ServerState::Failed { .. } => ui.colored_label(ui.visuals().error_fg_color, state),
        _ => ui.label(state),
    };
    ui.label(viewers_text(status.viewers));
    if status.viewers > 0 {
        ui.label(request_text(&status.request));
    }
    if let Some(report) = &status.report {
        ui.label(format!("Отчёт ТВ: {report}"));
    }
}

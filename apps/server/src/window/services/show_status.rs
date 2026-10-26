use crate::server::{ServerState, ServerStatus};
use crate::window::{request_text, show_failure, status_color, status_dot, status_headline};
use eframe::egui::{Align, Layout, RichText, Ui};

pub fn show_status(ui: &mut Ui, status: &ServerStatus) -> bool {
    ui.horizontal(|ui| {
        ui.label(RichText::new("FastCast").heading().strong());
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(status_headline(status)).strong());
            let color = status_color(&status.state, ui.visuals());
            status_dot(ui, color);
        });
    });

    let retry = match &status.state {
        ServerState::Failed { reason } => show_failure(ui, reason),
        _ => false,
    };
    if status.viewers > 0 {
        ui.label(RichText::new(request_text(&status.request)).weak());
    }
    if let Some(report) = &status.report {
        ui.label(RichText::new(format!("Отчёт ТВ: {report}")).weak());
    }

    retry
}

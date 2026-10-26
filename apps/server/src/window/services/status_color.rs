use crate::server::ServerState;
use crate::window::RUNNING_COLOR;
use eframe::egui::{Color32, Visuals};

pub fn status_color(state: &ServerState, visuals: &Visuals) -> Color32 {
    match state {
        ServerState::Running { .. } => RUNNING_COLOR,
        ServerState::Failed { .. } => visuals.error_fg_color,
        ServerState::Starting | ServerState::Stopped => visuals.weak_text_color(),
    }
}

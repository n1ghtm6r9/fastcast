use crate::window::STATUS_DOT_SIZE;
use eframe::egui::{Color32, Sense, Ui, Vec2};

pub fn status_dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(STATUS_DOT_SIZE), Sense::hover());

    ui.painter()
        .circle_filled(rect.center(), STATUS_DOT_SIZE / 2.0, color);
}

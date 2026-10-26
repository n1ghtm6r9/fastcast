use crate::app::{WINDOW_MIN_SIZE, WINDOW_SIZE, WINDOW_TITLE, window_icon};
use eframe::NativeOptions;
use eframe::egui::ViewportBuilder;

pub fn window_options() -> NativeOptions {
    let viewport = ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_inner_size(WINDOW_SIZE)
        .with_min_inner_size(WINDOW_MIN_SIZE);

    NativeOptions {
        viewport: match window_icon() {
            Some(icon) => viewport.with_icon(icon),
            None => viewport,
        },
        centered: true,
        ..NativeOptions::default()
    }
}

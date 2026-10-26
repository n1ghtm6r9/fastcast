use crate::app::{FASTCAST_ICON, largest_icon_picture};
use eframe::egui::IconData;
use eframe::icon_data::from_png_bytes;

pub fn window_icon() -> Option<IconData> {
    largest_icon_picture(FASTCAST_ICON).and_then(|picture| from_png_bytes(picture).ok())
}

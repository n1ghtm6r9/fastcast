use crate::settings::constants::VIDEO_BITRATE_MBPS_SETTING;
use crate::window::SLIDER_WIDTH;
use eframe::egui::{Response, Slider, Ui};

pub fn bitrate_slider(ui: &mut Ui, mbps: &mut u32) -> Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().slider_width = SLIDER_WIDTH;
        let slider = ui.add(
            Slider::new(mbps, VIDEO_BITRATE_MBPS_SETTING.range())
                .show_value(false)
                .trailing_fill(true),
        );
        ui.label(format!("{mbps} Мбит/с"));
        slider
    })
    .inner
}

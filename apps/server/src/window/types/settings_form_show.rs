use crate::settings::constants::DEFAULT_SERVER_NAME;
use crate::window::{
    CODEC_CHOICES, FIELD_WIDTH, FRAME_RATE_CHOICES, SettingsForm, bitrate_slider, choice_combo,
    screen_combo, setting_row,
};
use eframe::egui::{Grid, TextEdit, Ui};

impl SettingsForm {
    pub fn show(&mut self, ui: &mut Ui) {
        let edited = &mut self.edited;
        let monitors = &mut self.monitors;
        let list_monitors = self.list_monitors;

        self.editing = Grid::new("settings")
            .num_columns(2)
            .spacing([24.0, 12.0])
            .show(ui, |ui| {
                let name = setting_row(ui, "Имя на ТВ", |ui| {
                    ui.add(
                        TextEdit::singleline(&mut edited.server_name)
                            .hint_text(DEFAULT_SERVER_NAME)
                            .desired_width(FIELD_WIDTH),
                    )
                });
                setting_row(ui, "Экран", |ui| {
                    screen_combo(ui, edited, monitors, list_monitors)
                });
                setting_row(ui, "Кадров/с", |ui| {
                    choice_combo(
                        ui,
                        "frame_rate",
                        &mut edited.frame_rate,
                        &FRAME_RATE_CHOICES,
                    )
                });
                setting_row(ui, "Кодек", |ui| {
                    choice_combo(ui, "video_codec", &mut edited.video_codec, &CODEC_CHOICES)
                });
                let bitrate = setting_row(ui, "Битрейт", |ui| {
                    bitrate_slider(ui, &mut edited.video_bitrate_mbps)
                });

                name.has_focus() || bitrate.dragged()
            })
            .inner;

        if let Some(problem) = &self.problem {
            ui.colored_label(ui.visuals().error_fg_color, problem);
        }
    }
}
